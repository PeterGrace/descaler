use anyhow::{Result, bail};
use tokio::time::{Duration, interval};
use log::{info, warn, debug};
use backoff::ExponentialBackoff;
use backoff::future::retry;
use std::sync::{Arc, Mutex};
use crate::lib::config::{ScalerConfig, ScalerResource, AppConfig};
use kube::{
    api::{Api, ListParams},
    Client,
};
use k8s_openapi::api::core::v1::Node;
use k8s_openapi::api::autoscaling::v2beta1::HorizontalPodAutoscaler;
use futures::{StreamExt, TryStreamExt};
use std::collections::HashMap;
use kube_runtime::utils::try_flatten_applied;
use kube_runtime::watcher;
use kube::api::{Meta, ObjectMeta};

async fn remove_node_annotation(node: &Node)  {
    let mut annotations = node.metadata.annotations.clone().unwrap();

    if annotations.contains_key("vsix.me/descaler-enabled-at") {
        debug!("Found descaler-enabled-at key, deleting annotations for {}", Meta::name(node));
        annotations.remove("vsix.me/descaler-enabled-at");
        annotations.remove("cluster-autoscaler.kubernetes.io/scale-down-disabled");
    }
    else
    {
        debug!("No annotation found, leaving node alone: {}", Meta::name(node));
    }
}

async fn apply_node_annotation(node: &Node)  {
    warn!("Not implemented yet.");
}

pub async fn node_enumerate_loop(cfg: Arc<std::sync::Mutex<AppConfig>>,status: Arc<std::sync::Mutex<ScalerConfig>>) -> Result<()>{
    let mut interval = tokio::time::interval(Duration::from_secs(cfg.lock().unwrap().enumerate_nodes_secs as u64));
    let client = Client::try_default().await?;
    let lp = ListParams::default().allow_bookmarks();

    loop {
        interval.tick().await;
        if cfg.lock().unwrap().last_valid_scaler_config_at == 0 {
            info!("have not received valid scaler config yet, sleeping this node interval.");
            continue;
        }

        let scaling_enabled = status.lock().unwrap().scaling_enabled;
        if scaling_enabled {
            info!("Scaling is enabled, so will be removing annotations, if required.")
        } else {
            info!("Scaling is disabled, so we shall set annotations to stop scaling.")
        }

        let nodes: Api<Node> = Api::all(client.clone());
        for node in nodes.list(&lp).await? {
            debug!("Found node {}", Meta::name(&node));
            if scaling_enabled {
                remove_node_annotation(&node).await;
            }
            else {
                apply_node_annotation(&node).await;
            }
        }

    }
    debug!("node timer work completed,  awaiting next interval.");
    Ok(())

}