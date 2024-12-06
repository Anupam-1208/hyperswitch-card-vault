use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider};
use opentelemetry_sdk::{
    runtime::{self},
    Resource,
};
use std::time::Duration;

static INVOCATIONS: std::sync::Once = std::sync::Once::new();

pub(super) struct MetricsGuard {
    _metrics_guard: SdkMeterProvider,
}

pub fn init_metrics_provider() -> SdkMeterProvider {
    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317/v1/metrics") // get from env config
        .with_protocol(opentelemetry_otlp::Protocol::HttpJson)
        .with_timeout(Duration::from_secs(3))
        .build()
        .expect("Failed to build metrics exporter");

    let reader = PeriodicReader::builder(exporter, runtime::Tokio)
        .with_interval(std::time::Duration::from_secs(3))
        .with_timeout(Duration::from_secs(10))
        .build();

    let meter_provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            "Tartarus",
        )]))
        .build();

    global::set_meter_provider(meter_provider.clone());
    meter_provider
}


#[allow(clippy::expect_used)]
impl Drop for MetricsGuard {
    fn drop(&mut self) {
        self._metrics_guard
            .shutdown()
            .expect("Failed to shutdown the metrics pipeline")
    }
}