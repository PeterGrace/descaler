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

pub async fn scaler_enumerate_loop(cfg: Arc<std::sync::Mutex<AppConfig>> ,status: Arc<std::sync::Mutex<ScalerConfig>>) -> Result<()>{
    let mut interval = tokio::time::interval(Duration::from_secs(cfg.lock().unwrap().enumerate_scalers_secs as u64));
    let client = Client::try_default().await?;
    let lp = ListParams::default().allow_bookmarks();

    loop {
        interval.tick().await;
        debug!("scaler timer interval fired.");

        let hpas: Api<HorizontalPodAutoscaler> = Api::all(client.clone());
        for hpa in hpas.list(&lp).await? {
            debug!("Found hpa {}/{}", Meta::namespace(&hpa).unwrap(),Meta::name(&hpa));
        }
        debug!("scaler timer work completed,  awaiting next interval.");
    }
    Ok(())
}