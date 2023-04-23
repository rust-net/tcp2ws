use axum::*;
use tokio::*;
use response::IntoResponse;
use http::StatusCode;
use http::Method;
use sync::Mutex;

use super::*;
use config::*;

async fn config(method: Method, config: extract::State<std::sync::Arc<Mutex<Config>>>, payload: Option<Json<Config>>) -> impl IntoResponse {
    match method {
        Method::GET => {
            let config = config.lock().await;
            Ok((StatusCode::OK, Json(serde_json::json!(*config))))
        }
        Method::POST => {
            if let Some(Json(payload)) = payload {
                fs::write(CONFIG, serde_json::to_string_pretty(&payload).unwrap()).await.unwrap();
                let mut config = config.lock().await;
                *config = payload;
                Ok((StatusCode::OK, Json(serde_json::json!(*config))))
            } else {
                Err((StatusCode::BAD_REQUEST, "POST error, please check your payload"))
            }
        }
        _ => Err((StatusCode::METHOD_NOT_ALLOWED, "Only allow GET and POST requests"))
    }
}

async fn start(Json(item): Json<Item>) -> impl IntoResponse {
    match service::start(item).await {
        Ok(_) => Ok((StatusCode::CREATED, "服务启动成功")),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn stop(Json(item): Json<Item>) -> impl IntoResponse {
    match service::stop(item).await {
        Ok(_) => Ok("服务停止成功"),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn list() -> impl IntoResponse {
    match service::list().await {
        Ok(items) => Ok(Json(items)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub fn router() -> axum::Router<std::sync::Arc<Mutex<Config>>> {
    axum::Router::new()
        .route("/config", axum::routing::any(config))
        .route("/start", axum::routing::post(start))
        .route("/stop", axum::routing::post(stop))
        .route("/list", axum::routing::get(list))
}