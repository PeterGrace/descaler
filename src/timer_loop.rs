use anyhow::{Result, bail};
use tokio::prelude::*;
use tokio::time::{Duration, Instant, delay_for};
use log::{info, warn};
use again::RetryPolicy;
use crate::lib::config::{ScalerConfig, ScalerResource};
use kube::{api::{Api, Meta, Patch}, Client, Error};
use k8s_openapi::api::apps::v1::{Deployment, StatefulSet};
use tokio::macros::support::Future;

pub async fn create_and_start_timer_loop(url: String) -> anyhow::Result<()> {
    let policy = RetryPolicy::exponential(Duration::from_millis(100))
        .with_max_retries(10)
        .with_max_delay(Duration::from_secs(5));
    loop {
        delay_for(Duration::from_secs(10)).await;
        let fetch_call_result = policy.retry(|| async {
            let resp = match reqwest::get(&url).await {
                Ok(r) => r,
                Err(e) => bail!("Unable to get remote url: {:#?}", e)
            };
            match resp.text().await {
                Ok(t) => Ok(t),
                Err(e) => bail!("Could not get text from body: {:#?}", e)
            }
            }).await;
        let cfg:ScalerConfig = match fetch_call_result {
            Ok(c) => serde_yaml::from_str(c.as_str())?,
            Err(e) => bail!("Couldn't parse yaml from the remote site: {:#?}", e)
        };
        let k8s_client = kube::Client::try_default().await?;
        for res in cfg.objects {
            match res.kind.to_lowercase().as_str() {
                "deployment" => {
                    let obj: Api<Deployment> = Api::namespaced(k8s_client.clone(), res.namespace.as_str());
                    let named_deployment: Deployment = match obj.get(res.name.as_str()).await {
                        Ok(d) => d,
                        Err(e) => bail!("Could not find deployment as specified: {:#?}", e)
                    };
                    let replicaCount = match named_deployment.clone().spec.unwrap().replicas {
                        Some(num) => num,
                        None => bail!("unexpected response, no replicaCount on object {}?",res.name)
                    };
                    if (cfg.scaling_enabled == true)
                        && (replicaCount == 0) {
                        let mut new_obj = named_deployment.clone();
                        let mut new_spec = named_deployment.clone().spec.unwrap();
                        new_spec.replicas = Some(1);
                        new_obj.spec = Some(new_spec);
                        Patch::Apply(new_obj);
                    };
                    if (cfg.scaling_enabled == false)
                        && (replicaCount >= 1) {
                        let mut new_obj = named_deployment.clone();
                        let mut new_spec = named_deployment.clone().spec.unwrap();
                        new_spec.replicas = Some(0);
                        new_obj.spec = Some(new_spec);
                        Patch::Apply(new_obj);
                    };
                },
                "statefulset" => {
                    let obj: Api<StatefulSet> = Api::namespaced(k8s_client.clone(), res.namespace.as_str());
                },
                _ => bail!("Object did not have a valid kind defined: {}", res.kind)
            };
        }

        // check for file
    }
}