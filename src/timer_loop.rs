use anyhow::{Result, bail};
use tokio::time::{Duration, interval};
use log::{info, warn, debug};
use backoff;
use crate::lib::config::{ScalerConfig, ScalerResource};
use kube::{api::{Api, Patch}};
use k8s_openapi::api::apps::v1::{Deployment, StatefulSet};
use backoff::ExponentialBackoff;
use backoff::future::retry;

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let mut backoff = ExponentialBackoff::default();
    backoff.max_elapsed_time = Some(Duration::from_secs(5));
    retry(backoff, || async {
        Ok(reqwest::get(url).await?.text().await?)
    }).await
}

pub async fn create_and_start_timer_loop(url: String) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        debug!("Preparing to fetch url.");
        let fetch_call_result = match fetch_url(url.as_str()).await {
            Ok(s) => s,
            Err(e) => {
                if e.is_timeout() == true { warn!("timeout loading url {}.", url); }
                if e.is_connect() == true { warn!("error connecting to url {}.", url); }
                if e.is_status() == true { warn!("error retrieving data from url {}: http status {:#?}", url, e.status())}
                continue
            }
        };
        debug!("Got text, processing yaml.");
        let cfg:ScalerConfig = match serde_yaml::from_str(fetch_call_result.as_str()) {
            Ok(c) => c,
            Err(e) => bail!("Couldn't parse yaml from the remote site: {:#?}", e)
        };
        debug!("Creating k8s client.");
        let k8s_client = kube::Client::try_default().await?;
        for res in cfg.objects {
            match res.kind.to_lowercase().as_str() {
                "deployment" => {
                    let obj: Api<Deployment> = Api::namespaced(k8s_client.clone(), res.namespace.as_str());
                    let named_deployment: Deployment = match obj.get(res.name.as_str()).await {
                        Ok(d) => d,
                        Err(e) => bail!("Could not find deployment as specified: {:#?}", e)
                    };
                    let replica_count = match named_deployment.clone().spec.unwrap().replicas {
                        Some(num) => num,
                        None => bail!("unexpected response, no replicaCount on object {}?",res.name)
                    };
                    if (cfg.scaling_enabled == true)
                        && (replica_count == 0) {
                        let mut new_obj = named_deployment.clone();
                        let mut new_spec = named_deployment.clone().spec.unwrap();
                        new_spec.replicas = Some(1);
                        new_obj.spec = Some(new_spec);
                        Patch::Apply(new_obj);
                    };
                    if (cfg.scaling_enabled == false)
                        && (replica_count >= 1) {
                        let mut new_obj = named_deployment.clone();
                        let mut new_spec = named_deployment.clone().spec.unwrap();
                        new_spec.replicas = Some(0);
                        new_obj.spec = Some(new_spec);
                        Patch::Apply(new_obj);
                    };
                },
                "statefulset" => {
                    let obj: Api<StatefulSet> = Api::namespaced(k8s_client.clone(), res.namespace.as_str());
                    let named_deployment: StatefulSet = match obj.get(res.name.as_str()).await {
                        Ok(d) => d,
                        Err(e) => bail!("Could not find deployment as specified: {:#?}", e)
                    };
                    let replica_count = match named_deployment.clone().spec.unwrap().replicas {
                        Some(num) => num,
                        None => bail!("unexpected response, no replicaCount on object {}?",res.name)
                    };
                    if (cfg.scaling_enabled == true)
                        && (replica_count == 0) {
                        let mut new_obj = named_deployment.clone();
                        let mut new_spec = named_deployment.clone().spec.unwrap();
                        new_spec.replicas = Some(1);
                        new_obj.spec = Some(new_spec);
                        Patch::Apply(new_obj);
                    };
                    if (cfg.scaling_enabled == false)
                        && (replica_count >= 1) {
                        let mut new_obj = named_deployment.clone();
                        let mut new_spec = named_deployment.clone().spec.unwrap();
                        new_spec.replicas = Some(0);
                        new_obj.spec = Some(new_spec);
                        Patch::Apply(new_obj);
                    };
                },
                _ => bail!("Object did not have a valid kind defined: {}", res.kind)
            };
        }

        // check for file
    }
}