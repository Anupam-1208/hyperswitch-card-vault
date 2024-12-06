use tartarus::{logger, metric, metric::core::gather, tenant::GlobalAppState};

use axum::{routing::get, Router};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use std::sync::Arc;

#[allow(clippy::expect_used)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut global_config =
        tartarus::config::GlobalConfig::new().expect("Failed while parsing config");

    let _guard = logger::setup(
        &global_config.log,
        tartarus::service_name!(),
        [tartarus::service_name!(), "tower_http"],
    );
    let _metric_guard = metric::env::init_metrics_provider();

    #[allow(clippy::expect_used)]
    global_config
        .validate()
        .expect("Failed to validate application configuration");
    global_config
        .fetch_raw_secrets()
        .await
        .expect("Failed to fetch raw application secrets");

    let global_app_state = GlobalAppState::new(global_config).await;

    // tokio::task::spawn(spawn_metrics_server(global_app_state.clone()));
    tartarus::app::server_builder(global_app_state)
        .await
        .expect("Failed while building the server");

    // tartarus::app::span_metrics_server(global_app_state)
    //     .await
    //     .expect("Failed while building metrics server");
    Ok(())
}

async fn spawn_metrics_server(global_app_state: Arc<GlobalAppState>) {
    // let host: SocketAddr = format!(
    //     "{}:{}",
    //     &state.conf.metrics_server.host, &state.conf.metrics_server.port
    // )
    // .parse()
    // .expect("Unable to parse metrics server");
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = 8331;
    // let host: SocketAddr = SocketAddr::new(ip, port);
    logger::info!("Metrics Server started at {:?}:{:?}", ip, port,);
    let host: SocketAddr = format!("{}:{}", "127.0.0.1", port)
        .parse()
        .expect("unable to read server and port");

    let app = Router::new()
        .nest("/metrics", server(global_app_state.clone()))
        .with_state(global_app_state);

    axum_server::bind(host)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start the metrics server")
}

pub fn server(state: Arc<GlobalAppState>) -> Router<Arc<GlobalAppState>> {
    Router::new().route("/", get(gather)).with_state(state)
}
