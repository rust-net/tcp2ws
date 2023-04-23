use super::api;
use super::config::Config;

async fn exit() -> impl axum::response::IntoResponse {
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        std::process::exit(0);
    });
    "程序即将退出"
}

pub fn router(config: Config) -> axum::Router {
    axum::Router::new()
        .nest_service("/", tower_http::services::ServeDir::new("assets"))
        .route("/exit", axum::routing::get(exit))
        .nest("/api", api::router())
        // .with_state(config)
        .with_state(std::sync::Arc::new(tokio::sync::Mutex::new(config)))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::AllowOrigin::any())
                .allow_headers(tower_http::cors::Any)
        )
}