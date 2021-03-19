mod lib;
mod metrics;
mod status_management;
mod tests;
mod node_management;
mod scaler_management;

use std::fs;
use std::env;
use hyper::{
    Server,
    service::{make_service_fn, service_fn}
};
use anyhow::{bail, Result};
use log::info;
use lib::config::AppConfig;
use std::sync::{Arc, Mutex};
use crate::lib::config::ScalerConfig;
use crate::node_management::node_enumerate_loop;
use crate::scaler_management::scaler_enumerate_loop;
use crate::status_management::scaler_status_update_loop;


// populate auditable dependency structure for library chain-of-custody controls
static COMPRESSED_DEPENDENCY_LIST: &[u8] = auditable::inject_dependency_list!();

#[tokio::main]
async fn main() -> Result<()>{
    env_logger::init();
    let metrics_addr = ([0, 0, 0, 0], 9898).into();
    let serve_future = Server::bind(&metrics_addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(metrics::serve_metrics))
    }));
    let config_file_location = std::env::var("DESCALER_CONFIG_YAML").unwrap_or(String::from("config.yaml"));

    let appdata_gauge =
        metrics::APPVER.with_label_values(&[env!("CARGO_PKG_VERSION"), env!("GIT_HASH")]);
    appdata_gauge.set(1.0);
    tokio::spawn(async move { serve_future.await });
    info!("Service spawned, crate v{} hash:{}, auditable_dependency_payload_size:{:#?}", env!("CARGO_PKG_VERSION"), env!("GIT_HASH"), COMPRESSED_DEPENDENCY_LIST.to_vec().len());
    let config_raw = match fs::read_to_string(&config_file_location) {
        Ok(r) => r,
        Err(_) => bail!("Could not find config file at {}",config_file_location)
    };
    let config: AppConfig = match serde_yaml::from_str(config_raw.as_str()) {
        Ok(a) => a,
        Err(_) => bail!("Invalid config, check yaml contained in {}",config_file_location)
    };

    let cfg = Arc::new(Mutex::new(config));
    let status = Arc::new(Mutex::new(ScalerConfig::default()));
    match tokio::join!(
        scaler_status_update_loop(cfg.clone(), status.clone()),
        node_enumerate_loop(cfg.clone(), status.clone()),
        scaler_enumerate_loop(cfg.clone(), status.clone())
    ){
        (Err(e),_,_) => bail!("Error in scaler update loop: {}", e),
        (_,Err(e),_) => bail!("Error in node enumeration loop: {}", e),
        (_,_,Err(e)) => bail!("error in scaler object enumeration loop: {}", e),
        (_,_,_) => Ok(())
    }

}