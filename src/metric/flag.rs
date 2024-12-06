use once_cell::sync::Lazy;
use opentelemetry::metrics::{Counter, Histogram, Meter};

pub static METER: Lazy<Meter> = Lazy::new(|| opentelemetry::global::meter("card-vault"));

pub static HEALTH_METRIC: Lazy<Counter<u64>> =
    Lazy::new(|| METER.u64_counter("HEALTH_METRIC").build());

pub static ADD_CARD_FAILURE: Lazy<Counter<u64>> =
    Lazy::new(|| METER.u64_counter("ADD_CARD_FAILURE").build());

pub static RETRIEVE_CARD_FAILURE: Lazy<Counter<u64>> =
    Lazy::new(|| METER.u64_counter("RETRIEVE_CARD_FAILURE").build());

pub static DELETE_CARD_FAILURE: Lazy<Counter<u64>> =
    Lazy::new(|| METER.u64_counter("DELETE_CARD_FAILURE").build());

pub static FINGERPRINT_FAILURE: Lazy<Counter<u64>> =
    Lazy::new(|| METER.u64_counter("FINGERPRINT_FAILURE").build());

pub static ADD_CARD_API_LATENCY: Lazy<Histogram<f64>> =
    Lazy::new(|| METER.f64_histogram("ADD_CARD_API_LATENCY").build());

pub static RETRIEVE_CARD_API_LATENCY: Lazy<Histogram<f64>> =
    Lazy::new(|| METER.f64_histogram("RETRIEVE_CARD_API_LATENCY").build());

pub static DELETE_CARD_API_LATENCY: Lazy<Histogram<f64>> =
    Lazy::new(|| METER.f64_histogram("DELETE_CARD_API_LATENCY").build());

pub static FINGERPRINT_API_LATENCY: Lazy<Histogram<f64>> =
    Lazy::new(|| METER.f64_histogram("FINGERPRINT_API_LATENCY").build());

pub static HEALTH_API_LATENCY: Lazy<Histogram<f64>> =
    Lazy::new(|| METER.f64_histogram("HEALTH_API_LATENCY").build());