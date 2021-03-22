use crate::lib::config::{AppConfig, ScalerConfig};
use anyhow::Result;
use k8s_openapi::api::core::v1::Node;
use kube::api::{Meta, ObjectMeta, Patch};
use kube::{
    api::{Api, ListParams, PatchParams},
    Client,
};
use log::{debug, info, warn};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::time::Duration;

async fn remove_node_annotation(node: &mut Node) -> bool {
    let mut annotations = node.metadata.annotations.clone().unwrap();

    if annotations.contains_key("vsix.me/descaler-enabled-at") {
        debug!(
            "Found descaler-enabled-at key, deleting annotations for {}",
            Meta::name(node)
        );
        annotations.remove("vsix.me/descaler-enabled-at");
        annotations.remove("cluster-autoscaler.kubernetes.io/scale-down-disabled");
        node.metadata.annotations = Some(annotations);
        return true;
    }
    return false;
}

async fn apply_node_annotation(node: &mut Node) -> bool {
    let mut annotations = node.metadata.annotations.clone().unwrap();

    if !annotations.contains_key("cluster-autoscaler.kubernetes.io/scale-down-disabled") {
        debug!(
            "Adding scale-down-disable annotation to node {}",
            Meta::name(node)
        );
        annotations.insert(
            String::from("cluster-autoscaler.kubernetes.io/scale-down-disabled"),
            String::from("true"),
        );
        annotations.insert(
            String::from("vsix.me/descaler-enabled-at"),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
        );
        node.metadata.annotations = Some(annotations);
        return true;
    };
    return false;
}

pub async fn node_enumerate_loop(
    cfg: Arc<std::sync::Mutex<AppConfig>>,
    status: Arc<std::sync::Mutex<ScalerConfig>>,
) -> Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(
        cfg.lock().unwrap().enumerate_nodes_secs as u64,
    ));
    let client = match Client::try_default().await {
        Ok(c) => c,
        Err(e) => panic!("Unable to connect to kubernetes backend.  This is fatal.  Error: {:?}", e)
    };
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
        for mut node in nodes.list(&lp).await? {
            debug!("Found node {}", Meta::name(&node));
            let changes: bool;
            if scaling_enabled {
                changes = remove_node_annotation(&mut node).await;
            } else {
                changes = apply_node_annotation(&mut node).await;
            }
            if changes {
                match nodes
                    .patch(
                        node.metadata.name.as_ref().unwrap(),
                        &PatchParams::apply("descaler"),
                        &Patch::Apply({
                            Node {
                                metadata: ObjectMeta {
                                    annotations: Some(node.metadata.annotations.unwrap()),
                                    ..ObjectMeta::default()
                                },
                                ..Node::default()
                            }
                        }),
                    )
                    .await
                {
                    Ok(_) => {
                        info!("Node modified: {}", node.metadata.name.unwrap());
                    }
                    Err(e) => {
                        warn!(
                            "Could not amend node {}: {:?}",
                            node.metadata.name.unwrap(),
                            e
                        );
                    }
                }
            }
        }
    }
}
