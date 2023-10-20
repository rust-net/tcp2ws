use super::api;
use super::config::Config;

async fn exit() -> impl axum::response::IntoResponse {
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        std::process::exit(0);
    });
    "程序即将退出"
}

fn route_assets() -> axum::Router {
    let assets = macro_log::read_dir!("assets");
    let mut router = axum::Router::new();
    for (path, bin) in assets {
        #[cfg(windows)]
        let path = format!("/{}", path.replace("\\", "/"));
        macro_log::i!("serve: {path}");
        let mime = match () {
            _ if path.ends_with(".html") => axum::response::TypedHeader(axum::headers::ContentType::html()),
            _ if path.ends_with(".css") => axum::response::TypedHeader(axum::headers::ContentType::from(mime::TEXT_CSS)),
            _ if path.ends_with(".js") => axum::response::TypedHeader(axum::headers::ContentType::from(mime::TEXT_JAVASCRIPT)),
            _ if path.ends_with(".json") => axum::response::TypedHeader(axum::headers::ContentType::from(mime::APPLICATION_JSON)),
            _ => axum::response::TypedHeader(axum::headers::ContentType::octet_stream()),
        };
        let handler = || async {
            (mime, bin.as_ref())
        };
        if path.ends_with("index.html") {
            let path = &path[.. path.len() - "index.html".len()];
            router = router.route(&path, axum::routing::get(handler.clone()));
        }
        router = router.route(&path, axum::routing::get(handler));
    }
    router
}

pub fn router(config: Config) -> axum::Router {
    axum::Router::new()
        // .nest_service("/", tower_http::services::ServeDir::new("assets"))
        .nest_service("/", route_assets())
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