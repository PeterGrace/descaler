use hyper::{header::CONTENT_TYPE, Body, Request, Response};
use lazy_static::lazy_static;
use prometheus::{
    Encoder, TextEncoder,
    GaugeVec, register_gauge_vec,
    HistogramVec, register_histogram_vec,
    CounterVec, register_counter_vec,
    exponential_buckets
};

lazy_static! {

    // create a static prometheus metric for the app and git hash version.
    pub(crate) static ref METRIC_APPVER: GaugeVec = register_gauge_vec!(
        "descaler_app_info",
        "static app labels that potentially only change at restart",
        &["crate_version", "git_hash"]
    ).unwrap();

    // create a metric for tracking how long it takes to update a kubernetes object
    pub(crate) static ref METRIC_PATCH_DURATION: HistogramVec = register_histogram_vec!(
    "descaler_patch_update_duration",
    "the amount of time it takes to update a single node",
    &["object_type"],
    exponential_buckets(5.0, 1.5, 20).unwrap()
    ).unwrap();

    // create a counter metric for successful and failure w/r/t state changes
    pub(crate) static ref METRIC_PATCH_SUCCESS: CounterVec = register_counter_vec!(
    "descaler_patch_success_count",
    "the count of objects successfully patched thus far",
    &["scaling_enabled", "object_type"]
    ).unwrap();

    pub(crate) static ref METRIC_PATCH_FAILURE: CounterVec = register_counter_vec!(
    "descaler_patch_failure_count",
    "the count of objects that failed to patch properly",
    &["scaling_enabled", "object_type"]
    ).unwrap();
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