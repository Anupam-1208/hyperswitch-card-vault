use crate::error::{self, ContainerError};
use crate::tenant::GlobalAppState;
use axum::extract::State;
use error_stack::ResultExt;
use prometheus::{default_registry, Encoder, TextEncoder};
use std::{io::BufWriter, sync::Arc};
use once_cell::sync::Lazy;
use opentelemetry::{KeyValue,metrics::Histogram};
use std::time;

pub async fn gather(State(_): State<Arc<GlobalAppState>>) -> Vec<u8> {
    let registry = default_registry();
    let metrics_families = registry.gather();
    let encoder = TextEncoder::new();
    let mut buffer = BufWriter::new(Vec::new());

    encoder
        .encode(&metrics_families, &mut buffer)
        .change_context(error::ApiError::TenantError(
            "Failed to build the metrics encoder",
        ))
        .expect("Failed to build the metrics encoder");

    buffer
        .into_inner()
        .change_context(error::ApiError::TenantError(
            "Failed to flush the metrics buffer",
        ))
        .expect("Failed to flush the metrics buffer")
}


// helper function to track function latency
pub(crate) async fn record_operation_latency<F, T>(
    fut: F, // future function that needs to be called
    metric: &Lazy<Histogram<f64>>,// metric to be increasted
    key_value: &[KeyValue],
) -> Result<T, ContainerError<error::ApiError>>
where
    F: futures::Future<Output = Result<T, ContainerError<error::ApiError>>>,
{
    let time = time::Instant::now();
    let result = fut.await;
    let elapsed: time::Duration = time.elapsed();
    metric.record(elapsed.as_secs_f64(), key_value);
    result
}
