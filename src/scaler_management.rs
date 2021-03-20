use anyhow::{Result, bail};
use tokio::time::{Duration, interval};
use log::{info, warn, debug};
use backoff::ExponentialBackoff;
use backoff::future::retry;
use std::sync::{Arc, Mutex};
use crate::lib::config::{ScalerConfig, ScalerResource, AppConfig};
use openapi_scaledobjects::models::ScaledObject;
use openapi_scaledjobs::models::ScaledJob;
use kube::{
    api::{Api, ListParams},
    Client
};
use k8s_openapi::api::core::v1::Node;
use k8s_openapi::api::autoscaling::v2beta1::HorizontalPodAutoscaler;
use futures::{StreamExt, TryStreamExt};
use std::collections::HashMap;
use kube_runtime::utils::try_flatten_applied;
use kube_runtime::watcher;
use kube::api::{Meta, ObjectMeta, Patch, PatchParams};
use serde_json::json;
use openapi_scaledobjects::models::scaled_object::ScaledObjectSpec;

pub async fn scaler_enumerate_loop(cfg: Arc<std::sync::Mutex<AppConfig>> ,status: Arc<std::sync::Mutex<ScalerConfig>>) -> Result<()>{
    let mut interval = tokio::time::interval(Duration::from_secs(cfg.lock().unwrap().enumerate_scalers_secs as u64));
    let client = Client::try_default().await?;
    let lp = ListParams::default().allow_bookmarks();

    loop {
        interval.tick().await;
        if cfg.lock().unwrap().last_valid_scaler_config_at == 0 {
            info!("have not received valid scaler config yet, sleeping this node interval.");
            continue;
        }
        debug!("scaler timer interval fired.");
        let scaling_enabled = status.lock().unwrap().scaling_enabled;

        let hpas: Api<HorizontalPodAutoscaler> = Api::all(client.clone());
        for hpa in hpas.list(&lp).await? {
            let mut scaler_type: ScalerType = ScalerType::HPA;
            if hpa.metadata.owner_references.is_some() {
                for owner in  hpa.metadata.owner_references.clone().unwrap() {
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
                    process_keda_scaled_object(client.clone(), hpa.clone(),s, scaling_enabled).await;
                },
                ScalerType::ScaledJob(s) => {
                    debug!("Found a hpa backed by a ScaledJob, actioning on that instead.");
                    process_keda_scaled_object(client.clone(), hpa.clone(),s, scaling_enabled).await;
                }
                ScalerType::HPA => {
                    debug!("This is a bare HPA without keda, actioning on this object.");
                    process_hpa(&hpa, scaling_enabled).await;
                }
            }
        }
    }
    debug!("scaler timer work completed,  awaiting next interval.");
    Ok(())
}
async fn process_hpa(hpa: &HorizontalPodAutoscaler, scaling_enabled: bool) {
    warn!("not implemented");
}

async fn process_keda_scaled_object(client: Client, hpa: HorizontalPodAutoscaler,name: String, scaling_enabled: bool) {
    let scaled_objects: Api<ScaledObject> = Api::namespaced(client, Meta::namespace(&hpa).unwrap().as_str());
    let our_object: ScaledObject = match scaled_objects.get(name.as_str()).await {
        Ok(obj) => (obj),
        Err(e) => {
            warn!("Unable to get ScaledObject {}/{}: {:?}", Meta::namespace(&hpa).unwrap(), name, e);
            return;
        }
    };
    debug!("Got scaled object {}/{}",Meta::namespace(&our_object).unwrap(),Meta::name(&our_object));
    let mut annotations = our_object.metadata.annotations.clone().unwrap();
    let current_replicas = hpa.status.unwrap().current_replicas;
    if scaling_enabled {
        if annotations.contains_key("vsix.me/descaler-original-replicas") {
            debug!("Found original replicas key, adjusting values.");
            let replica_count = annotations.get("vsix.me/descaler-original-replicas").unwrap().parse::<i32>().unwrap();
            annotations.remove("vsix.me/descaler-original-replicas");
            let patch = Patch::Apply(
                ScaledObject {
                    metadata: ObjectMeta {
                        annotations: Some(annotations),
                        .. ObjectMeta::default()
                    },
                    spec: ScaledObjectSpec {
                        triggers: our_object.spec.triggers,
                        scale_target_ref: our_object.spec.scale_target_ref,
                        min_replica_count: Some(replica_count),
                        .. ScaledObjectSpec::default()
                    },
                        .. ScaledObject::default()
                }
            );
            match scaled_objects.patch(name.as_str(), &PatchParams::apply("descaler"), &patch).await {
                Ok(_) => {
                    info!("Scaling Enabled for ScaledObject {}/{}", hpa.metadata.namespace.unwrap(), name);
                },
                Err(e) => {
                    warn!("Unable to patch ScaledObject: {:?}", e);
                }

            }

        }
    } else {
        if !annotations.contains_key("vsix.me/descaler-original-replicas") {
            debug!("Found object that needs to have scaling locked.");
            annotations.insert(String::from("vsix.me/descaler-original-replicas"), our_object.spec.min_replica_count.unwrap().to_string());
            let patch = Patch::Apply(
                ScaledObject {
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
                        .. ScaledObject::default()
                }
            );

            match scaled_objects.patch(name.as_str(), &PatchParams::apply("descaler"), &patch).await {
                Ok(_) => {
                    info!("Scaling Disabled for ScaledObject {}/{}", hpa.metadata.namespace.unwrap(), name);
                },
                Err(e) => {
                    warn!("Unable to patch ScaledObject: {:?}", e);
                }
            }
        }
    }



}
async fn process_keda_scaled_job(client: Client, hpa: HorizontalPodAutoscaler,name: String, scaling_enabled: bool) {
    warn!("not implemented");
}


enum ScalerType {
    ScaledObject(String),
    ScaledJob(String),
    HPA
}