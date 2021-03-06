use crate::lib::config::{AppConfig, ScalerConfig};
use anyhow::Result;
use backoff::future::retry;
use backoff::ExponentialBackoff;
use log::{debug, warn};
use std::fs;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::time::Duration;

pub async fn fetch_url(url: &str) -> std::result::Result<String, reqwest::Error> {
    if url.to_string().starts_with("file://") {
        let filename = url.strip_prefix("file://").unwrap();
        match fs::read_to_string(filename) {
            Ok(s) => Ok(s),
            Err(e) => {
                panic!(
                    "tried to read scaler contents from filesystem but failed.  This is fatal: {}",
                    e
                )
            }
        }
    } else {
        let mut backoff = ExponentialBackoff::default();
        backoff.max_elapsed_time = Some(Duration::from_secs(5));
        retry(backoff, || async {
            Ok(reqwest::get(url).await?.text().await?)
        })
            .await
    }
}

pub async fn scaler_status_update_loop(
    cfg: Arc<std::sync::Mutex<AppConfig>>,
    status: Arc<std::sync::Mutex<ScalerConfig>>,
) -> Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(
        cfg.clone().lock().unwrap().check_interval as u64,
    ));
    loop {
        interval.tick().await;
        debug!("Preparing to fetch url.");
        let url: String = cfg.clone().lock().unwrap().source_url.clone();
        let fetch_call_result = match fetch_url(url.as_str()).await {
            Ok(s) => s,
            Err(e) => {
                if e.is_timeout() {
                    warn!("timeout loading url {}: {:?}.", url, e);
                }
                if e.is_connect() {
                    warn!("error connecting to url {}: {:?}.", url, e);
                }
                if e.is_status() {
                    warn!(
                        "error retrieving data from url {}: http status {:#?}",
                        url,
                        e.status()
                    )
                }
                continue;
            }
        };
        debug!("Got text, processing yaml.");
        let recv_cfg: ScalerConfig = match serde_yaml::from_str(fetch_call_result.as_str()) {
            Ok(c) => c,
            Err(e) => {
                warn!("Couldn't parse yaml from the remote site: {}", e);
                continue;
            }
        };
        debug!("Setting value.");

        status.lock().unwrap().scaling_enabled = recv_cfg.scaling_enabled;
        cfg.lock().unwrap().last_valid_scaler_config_at = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}
