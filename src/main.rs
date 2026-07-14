mod config;
mod routers;
mod state;
mod vides;

use crate::config::AppConfig;
use crate::routers::build_global_routers;
use crate::state::AppState;
use clap::Parser;
use std::sync::Arc;
use mimalloc::MiMalloc;
use tracing::{info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi_sanitization(true)
                .with_line_number(true)
                .pretty(),
        )
        .init();
    let app_config: AppConfig = AppConfig::parse();
    info!("{:#?}", &app_config);
    let app_listener =
        tokio::net::TcpListener::bind(format!("{}:{}", &app_config.ipv4, &app_config.port))
            .await
            .expect("TcpListener Bind Error");

    info!(
        "Listening at http://{}:{}",
        local_ip_address::local_ip().map_or_else(|_| app_config.ipv4.clone(), |ip| ip.to_string()),
        app_config.port
    );
    let app_state = AppState {
        config: Arc::new(app_config),
    };
    let app_routers = build_global_routers(app_state);
    axum::serve(
        app_listener,
        app_routers.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .expect("Start Axum Server Error");
}
