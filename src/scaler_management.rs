use crate::lib::config::{AppConfig, ScalerConfig, ScalerType};
use anyhow::Result;
use k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler;
use k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscalerSpec;
use kube::api::{Meta, ObjectMeta, Patch, PatchParams};
use kube::{
    api::{Api, ListParams},
    Client,
};
use log::{debug, info, warn};
//use openapi_scaledjobs::models::ScaledJob;
use openapi_scaledobjects::models::scaled_object::ScaledObjectSpec;
use openapi_scaledobjects::models::ScaledObject;
use std::sync::Arc;
use tokio::time::Duration;
use crate::metrics::{METRIC_PATCH_DURATION, METRIC_PATCH_SUCCESS, METRIC_PATCH_FAILURE};
use std::time::SystemTime;


pub async fn scaler_enumerate_loop(
    cfg: Arc<std::sync::Mutex<AppConfig>>,
    status: Arc<std::sync::Mutex<ScalerConfig>>,
) -> Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(
        cfg.lock().unwrap().enumerate_scalers_secs as u64,
    ));
    debug!("enter scaler_enumerate_loop");
    let client = match Client::try_default().await {
        Ok(c) => c,
        Err(e) => panic!("Unable to connect to kubernetes backend.  This is fatal.  Error: {:?}", e)
    };
    let lp = ListParams::default().allow_bookmarks();

    loop {
        interval.tick().await;
        if cfg.lock().unwrap().last_valid_scaler_config_at == 0 {
            info!("have not received valid scaler config yet, sleeping this scaler interval.");
            continue;
        }
        debug!("scaler timer interval fired.");
        let scaling_enabled = status.lock().unwrap().scaling_enabled;

        let hpas: Api<HorizontalPodAutoscaler> = Api::all(client.clone());
        for hpa in hpas.list(&lp).await.expect("Unable to list hpas") {
            let start = SystemTime::now();
            let mut scaler_type: ScalerType = ScalerType::HorizontalPodAutoscaler;
            if hpa.metadata.owner_references.is_some() {
                for owner in hpa.metadata.owner_references.clone().unwrap() {
                    scaler_type = match owner.kind.as_str() {
                        "ScaledObject" => ScalerType::ScaledObject(owner.name),
                        "ScaledJob" => ScalerType::ScaledJob(owner.name),
                        _ => {
                            warn!("Did not find an expected owner type for this hpa, skipping.");
                            continue;
                        }
                    }
                }
            }
            let mut updates_done: bool = false;
            match scaler_type.clone() {
                ScalerType::ScaledObject(s) => {
                    debug!("Found a hpa backed by a ScaledObject, actioning on that instead.");
                    updates_done = process_keda_scaled_object(client.clone(), hpa.clone(), s, scaling_enabled)
                        .await;
                }
                ScalerType::ScaledJob(s) => {
                    debug!("Found a hpa backed by a ScaledJob, actioning on that instead.");
                    updates_done = process_keda_scaled_job(client.clone(), hpa.clone(), s, scaling_enabled).await;
                }
                ScalerType::HorizontalPodAutoscaler => {
                    debug!("This is a bare HPA without keda, actioning on this object.");
                    updates_done = process_hpa(client.clone(), &hpa, scaling_enabled).await;
                }
                ScalerType::Node => {
                    warn!("We received a scaler_type of Node, but in object scaler handler code.  This should not happen.");
                }
            };
            let end = SystemTime::now();
            if updates_done {
                METRIC_PATCH_DURATION.with_label_values(&[scaler_type.as_ref()]).observe(end.duration_since(start).unwrap().as_millis() as f64);
            }
        }
    }
}

async fn process_hpa(client: Client, hpa: &HorizontalPodAutoscaler, scaling_enabled: bool) -> bool {
    let hpas: Api<HorizontalPodAutoscaler> =
        Api::namespaced(client, hpa.metadata.namespace.as_ref().unwrap().as_str());
    let mut annotations = hpa.metadata.annotations.clone().unwrap();

    if scaling_enabled {
        if annotations.contains_key("vsix.me/descaler-original-min-replicas") {
            debug!("Found original replicas key, adjusting values and removing annotation.");
            let original_min_replica_count = annotations
                .get("vsix.me/descaler-original-min-replicas")
                .unwrap()
                .parse::<i32>()
                .unwrap();
            annotations.remove("vsix.me/descaler-original-min-replicas");
            let patch = Patch::Apply(HorizontalPodAutoscaler {
                metadata: ObjectMeta {
                    annotations: Some(annotations),

                    cluster_name: None,
                    creation_timestamp: None,
                    deletion_grace_period_seconds: None,
                    deletion_timestamp: None,
                    finalizers: None,
                    generate_name: None,
                    generation: None,
                    labels: None,
                    managed_fields: None,
                    name: None,
                    namespace: None,
                    owner_references: None,
                    resource_version: None,
                    self_link: None,
                    uid: None,
                },
                spec: Some(HorizontalPodAutoscalerSpec {
                    max_replicas: hpa.clone().spec.unwrap().max_replicas,
                    min_replicas: Some(original_min_replica_count),
                    scale_target_ref: hpa.clone().spec.unwrap().scale_target_ref,
                    target_cpu_utilization_percentage: None,
                }),
                status: None,
            });
            match hpas
                .patch(
                    hpa.metadata.name.as_ref().unwrap().as_str(),
                    &PatchParams::apply("descaler"),
                    &patch,
                )
                .await
            {
                Ok(_) => {
                    METRIC_PATCH_SUCCESS.with_label_values(&[format!("{}", scaling_enabled).as_str(), ScalerType::HorizontalPodAutoscaler.as_ref()]).inc();
                    info!(
                        "Scaling Enabled for HPA {}/{}",
                        hpa.metadata.namespace.as_ref().unwrap(),
                        hpa.metadata.name.as_ref().unwrap()
                    );
                    return true
                }
                Err(e) => {
                    METRIC_PATCH_FAILURE.with_label_values(&[format!("{}", scaling_enabled).as_str(), "hpa"]).inc();
                    warn!("Unable to patch HPA: {:?}", e);
                    return false
                }
            }
        }
    } else {
        if !annotations.contains_key("vsix.me/descaler-original-min-replicas") {
            debug!("Found object that needs to have scaling locked; setting minrep=current and annotating.");
            let current_replicas = hpa.status.as_ref().unwrap().current_replicas;
            annotations.insert(
                String::from("vsix.me/descaler-original-min-replicas"),
                hpa.spec.as_ref().unwrap().min_replicas.unwrap().to_string(),
            );
            let patch = Patch::Apply(HorizontalPodAutoscaler {
                metadata: ObjectMeta {
                    annotations: Some(annotations),
                    cluster_name: None,
                    creation_timestamp: None,
                    deletion_grace_period_seconds: None,
                    deletion_timestamp: None,
                    finalizers: None,
                    generate_name: None,
                    generation: None,
                    labels: None,
                    managed_fields: None,
                    name: None,
                    namespace: None,
                    owner_references: None,
                    resource_version: None,
                    self_link: None,
                    uid: None,
                },
                spec: Some(HorizontalPodAutoscalerSpec {
                    max_replicas: hpa.clone().spec.unwrap().max_replicas,
                    min_replicas: Some(current_replicas),
                    scale_target_ref: hpa.clone().spec.unwrap().scale_target_ref,
                    target_cpu_utilization_percentage: None,
                }),
                status: None,
            });

            match hpas
                .patch(
                    hpa.metadata.name.as_ref().unwrap().as_str(),
                    &PatchParams::apply("descaler"),
                    &patch,
                )
                .await
            {
                Ok(_) => {
                    METRIC_PATCH_SUCCESS.with_label_values(&[format!("{}", scaling_enabled).as_str(), ScalerType::HorizontalPodAutoscaler.as_ref()]).inc();
                    info!(
                        "Scaling Disabled for HPA {}/{}",
                        hpa.metadata.namespace.as_ref().unwrap(),
                        hpa.metadata.name.as_ref().unwrap()
                    );
                    return true
                }
                Err(e) => {
                    METRIC_PATCH_FAILURE.with_label_values(&[format!("{}", scaling_enabled).as_str(), ScalerType::HorizontalPodAutoscaler.as_ref()]).inc();
                    warn!("Unable to patch HPA: {:?}", e);
                    return false
                }
            }
        }
    }
    false
}

async fn process_keda_scaled_object(
    client: Client,
    hpa: HorizontalPodAutoscaler,
    name: String,
    scaling_enabled: bool,
) -> bool {
    let scaled_objects: Api<ScaledObject> =
        Api::namespaced(client, Meta::namespace(&hpa).unwrap().as_str());
    let our_object: ScaledObject = match scaled_objects.get(name.as_str()).await {
        Ok(obj) => (obj),
        Err(e) => {
            METRIC_PATCH_FAILURE.with_label_values(&[format!("{}", scaling_enabled).as_str(), ScalerType::ScaledObject("".to_string()).as_ref()]).inc();
            warn!(
                "Unable to get ScaledObject {}/{}: {:?}",
                Meta::namespace(&hpa).unwrap(),
                name,
                e
            );
            return false;
        }
    };
    debug!(
        "Got scaled object {}/{}",
        Meta::namespace(&our_object).unwrap(),
        Meta::name(&our_object)
    );
    let mut annotations = our_object.metadata.annotations.clone().unwrap();
    if scaling_enabled {
        if annotations.contains_key("vsix.me/descaler-original-min-replicas") {
            debug!("Found original replicas key, adjusting values.");
            let original_min_replica_count = annotations
                .get("vsix.me/descaler-original-min-replicas")
                .unwrap()
                .parse::<i32>()
                .unwrap();
            annotations.remove("vsix.me/descaler-original-min-replicas");
            let patch = Patch::Apply(ScaledObject {
                metadata: ObjectMeta {
                    annotations: Some(annotations),
                    cluster_name: None,
                    creation_timestamp: None,
                    deletion_grace_period_seconds: None,
                    deletion_timestamp: None,
                    finalizers: None,
                    generate_name: None,
                    generation: None,
                    labels: None,
                    managed_fields: None,
                    name: None,
                    namespace: None,
                    owner_references: None,
                    resource_version: None,
                    self_link: None,
                    uid: None,
                },
                spec: ScaledObjectSpec {
                    advanced: None,
                    cooldown_period: None,
                    triggers: our_object.clone().spec.triggers,
                    scale_target_ref: our_object.clone().spec.scale_target_ref,
                    min_replica_count: Some(original_min_replica_count),
                    max_replica_count: None,
                    polling_interval: None,
                },
                api_version: our_object.clone().api_version,
                kind: our_object.clone().kind,
            });
            match scaled_objects
                .patch(name.as_str(), &PatchParams::apply("descaler"), &patch)
                .await
            {
                Ok(_) => {
                    METRIC_PATCH_SUCCESS.with_label_values(&[format!("{}", scaling_enabled).as_str(), ScalerType::ScaledObject("".to_string()).as_ref()]).inc();
                    info!(
                        "Scaling Enabled for ScaledObject {}/{}",
                        hpa.metadata.namespace.unwrap(),
                        name
                    );
                    return true;
                }
                Err(e) => {
                    METRIC_PATCH_FAILURE.with_label_values(&[format!("{}", scaling_enabled).as_str(), ScalerType::ScaledObject("".to_string()).as_ref()]).inc();
                    warn!("Unable to patch ScaledObject: {:?}", e);
                    return false;
                }
            }
        }
    } else {
        if !annotations.contains_key("vsix.me/descaler-original-min-replicas") {
            debug!("Found object that needs to have scaling locked.");
            let current_replicas = hpa.status.unwrap().current_replicas;
            annotations.insert(
                String::from("vsix.me/descaler-original-min-replicas"),
                our_object.spec.min_replica_count.unwrap().to_string(),
            );
            let patch = Patch::Apply(ScaledObject {
                metadata: ObjectMeta {
                    annotations: Some(annotations),
                    cluster_name: None,
                    creation_timestamp: None,
                    deletion_grace_period_seconds: None,
                    deletion_timestamp: None,
                    finalizers: None,
                    generate_name: None,
                    generation: None,
                    labels: None,
                    managed_fields: None,
                    name: None,
                    namespace: None,
                    owner_references: None,
                    resource_version: None,
                    self_link: None,
                    uid: None,
                },
                spec: ScaledObjectSpec {
                    advanced: None,
                    cooldown_period: None,
                    triggers: our_object.clone().spec.triggers,
                    scale_target_ref: our_object.clone().spec.scale_target_ref,
                    min_replica_count: Some(current_replicas),
                    max_replica_count: None,
                    polling_interval: None,
                },
                api_version: our_object.clone().api_version,
                kind: our_object.clone().kind,
            });
            match scaled_objects
                .patch(name.as_str(), &PatchParams::apply("descaler"), &patch)
                .await
            {
                Ok(_) => {
                    METRIC_PATCH_SUCCESS.with_label_values(&[format!("{}", scaling_enabled).as_str(), ScalerType::ScaledObject("".to_string()).as_ref()]).inc();
                    info!(
                        "Scaling Disabled for ScaledObject {}/{}",
                        hpa.metadata.namespace.unwrap(),
                        name
                    );
                    return true;
                }
                Err(e) => {
                    METRIC_PATCH_FAILURE.with_label_values(&[format!("{}", scaling_enabled).as_str(), ScalerType::ScaledObject("".to_string()).as_ref()]).inc();
                    warn!("Unable to patch ScaledObject: {:?}", e);
                    return false;
                }
            }
        }
    }
    false
}

async fn process_keda_scaled_job(
    _client: Client,
    _hpa: HorizontalPodAutoscaler,
    _name: String,
    _scaling_enabled: bool,
) -> bool {
    // as of march 2021, scaledjobs don't have a minreplicas so there's no point in continuing.
    // I'm leaving this stub here, however, in case they later do support such a thing, in which
    // case we'd need to do some further processing.
    info!("ScaledJob found.  They have no minreplicas to adjust as of March, 2021.");
    return false;
}