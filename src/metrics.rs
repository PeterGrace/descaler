use prometheus::{Encoder, GaugeVec, TextEncoder, register_gauge_vec};
use hyper::{
    header::CONTENT_TYPE,
    Body, Request, Response,
};
use lazy_static::lazy_static;

// create a static prometheus metric for the app and git hash version.
lazy_static! {
    pub(crate) static ref APPVER: GaugeVec = register_gauge_vec!(
        "app_info",
        "static app labels that potentially only change at restart",
        &["crate_version", "git_hash"]
    )
    .unwrap();
}

pub(crate) async fn serve_metrics(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();
    Ok(response)
}