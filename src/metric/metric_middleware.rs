use axum::{extract::Request, middleware::Next, response::Response,};
use opentelemetry::KeyValue;
use opentelemetry::metrics::Histogram;
use once_cell::sync::Lazy;
use std::time;


pub async fn metric_middleware(
    req: Request,
    next: Next,
    metric: &Lazy<Histogram<f64>>,
    key_value: Vec<KeyValue>,
) -> Response {
    let time = time::Instant::now();
    let response = next.run(req).await;
    let elapsed: time::Duration = time.elapsed();
    metric.record(elapsed.as_secs_f64(), &key_value);
    response
}