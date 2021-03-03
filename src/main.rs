mod lib;
mod metrics;
mod timer_loop;

use std::fs;
use std::env;
use hyper::{
    service::{make_service_fn, service_fn},
    Server
};
use anyhow::{bail, Result};
use log::info;
use lib::config::AppConfig;


// populate auditable dependency structure for library chain-of-custody controls
static COMPRESSED_DEPENDENCY_LIST: &[u8] = auditable::inject_dependency_list!();

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    std::env::set_var("RUST_LOG", "info,kube=info");
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
        Err(e) => bail!("Could not find config file at {}",config_file_location)
    };
    let config: AppConfig = match serde_yaml::from_str(config_raw.as_str()) {
        Ok(a) => a,
        Err(e) => bail!("Invalid config, check yaml contained in {}",config_file_location)
    };
    let result = timer_loop::create_and_start_timer_loop(config.source_url).await;
    Ok(())
}