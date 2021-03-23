use crate::lib::config::{AppConfig, ScalerConfig};
use anyhow::Result;
use k8s_openapi::api::autoscaling::v2beta1::HorizontalPodAutoscaler;
use k8s_openapi::api::autoscaling::v2beta1::HorizontalPodAutoscalerSpec;
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
        for hpa in hpas.list(&lp).await? {
            let mut scaler_type: ScalerType = ScalerType::HPA;
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
            match scaler_type {
                ScalerType::ScaledObject(s) => {
                    debug!("Found a hpa backed by a ScaledObject, actioning on that instead.");
                    process_keda_scaled_object(client.clone(), hpa.clone(), s, scaling_enabled)
                        .await;
                }
                ScalerType::ScaledJob(s) => {
                    debug!("Found a hpa backed by a ScaledJob, actioning on that instead.");
                    process_keda_scaled_job(client.clone(), hpa.clone(), s, scaling_enabled).await;
                }
                ScalerType::HPA => {
                    debug!("This is a bare HPA without keda, actioning on this object.");
                    process_hpa(client.clone(), &hpa, scaling_enabled).await;
                }
            }
        }
    }
}

async fn process_hpa(client: Client, hpa: &HorizontalPodAutoscaler, scaling_enabled: bool) {
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
                    ..ObjectMeta::default()
                },
                spec: Some(HorizontalPodAutoscalerSpec {
                    min_replicas: Some(original_min_replica_count),
                    ..HorizontalPodAutoscalerSpec::default()
                }),
                ..HorizontalPodAutoscaler::default()
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
                    info!(
                        "Scaling Enabled for HPA {}/{}",
                        hpa.metadata.namespace.as_ref().unwrap(),
                        hpa.metadata.name.as_ref().unwrap()
                    );
                }
                Err(e) => {
                    warn!("Unable to patch ScaledObject: {:?}", e);
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
                    ..ObjectMeta::default()
                },
                spec: Some(HorizontalPodAutoscalerSpec {
                    min_replicas: Some(current_replicas),
                    ..HorizontalPodAutoscalerSpec::default()
                }),
                ..HorizontalPodAutoscaler::default()
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
                    info!(
                        "Scaling Disabled for HPA {}/{}",
                        hpa.metadata.namespace.as_ref().unwrap(),
                        hpa.metadata.name.as_ref().unwrap()
                    );
                }
                Err(e) => {
                    warn!("Unable to patch ScaledObject: {:?}", e);
                }
            }
        }
    }
}

async fn process_keda_scaled_object(
    client: Client,
    hpa: HorizontalPodAutoscaler,
    name: String,
    scaling_enabled: bool,
) {
    let scaled_objects: Api<ScaledObject> =
        Api::namespaced(client, Meta::namespace(&hpa).unwrap().as_str());
    let our_object: ScaledObject = match scaled_objects.get(name.as_str()).await {
        Ok(obj) => (obj),
        Err(e) => {
            warn!(
                "Unable to get ScaledObject {}/{}: {:?}",
                Meta::namespace(&hpa).unwrap(),
                name,
                e
            );
            return;
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
                    ..ObjectMeta::default()
                },
                spec: ScaledObjectSpec {
                    triggers: our_object.spec.triggers,
                    scale_target_ref: our_object.spec.scale_target_ref,
                    min_replica_count: Some(original_min_replica_count),
                    ..ScaledObjectSpec::default()
                },
                ..ScaledObject::default()
            });
            match scaled_objects
                .patch(name.as_str(), &PatchParams::apply("descaler"), &patch)
                .await
            {
                Ok(_) => {
                    info!(
                        "Scaling Enabled for ScaledObject {}/{}",
                        hpa.metadata.namespace.unwrap(),
                        name
                    );
                }
                Err(e) => {
                    warn!("Unable to patch ScaledObject: {:?}", e);
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
                    ..ObjectMeta::default()
                },
                spec: ScaledObjectSpec {
                    triggers: our_object.spec.triggers,
                    scale_target_ref: our_object.spec.scale_target_ref,
                    min_replica_count: Some(current_replicas),
                    ..ScaledObjectSpec::default()
                },
                ..ScaledObject::default()
            });

            match scaled_objects
                .patch(name.as_str(), &PatchParams::apply("descaler"), &patch)
                .await
            {
                Ok(_) => {
                    info!(
                        "Scaling Disabled for ScaledObject {}/{}",
                        hpa.metadata.namespace.unwrap(),
                        name
                    );
                }
                Err(e) => {
                    warn!("Unable to patch ScaledObject: {:?}", e);
                }
            }
        }
    }
}

async fn process_keda_scaled_job(
    _client: Client,
    _hpa: HorizontalPodAutoscaler,
    _name: String,
    _scaling_enabled: bool,
) {
    // as of march 2021, scaledjobs don't have a minreplicas so there's no point in continuing.
    // I'm leaving this stub here, however, in case they later do support such a thing, in which
    // case we'd need to do some further processing.
    info!("ScaledJob found.  They have no minreplicas to adjust as of March, 2021.");
}

enum ScalerType {
    ScaledObject(String),
    ScaledJob(String),
    HPA,
}
