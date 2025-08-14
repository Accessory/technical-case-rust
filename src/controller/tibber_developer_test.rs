use std::sync::Arc;

use axum::{
    extract::{self, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use tracing::warn;

use crate::{
    models::robot_status::RobotStatus,
    requests::enter_path_request::EnterPathRequest,
    service::robot_service::{self, RobotService},
};

pub fn new_router(robot_service: Arc<RobotService>) -> Router {
    Router::new()
        .route("/enter-path", post(enter_path))
        .route("/hallo-welt", get(hallo_welt))
        .with_state(robot_service)
}

#[utoipa::path(
    post,
    path = "/enter-path",
    request_body = EnterPathRequest,
    responses(
        (status = 200, description = "Hallo Welt", body = RobotStatus),
    ),
    context_path = "/tibber-developer-test"
)]
#[axum::debug_handler]
pub async fn enter_path(
    State(robot_service): State<Arc<RobotService>>,
    extract::Json(req): extract::Json<EnterPathRequest>,
) -> Result<Json<RobotStatus>, StatusCode> {
    let robot_status: RobotStatus = robot_service::RobotService::run_path(req);
    match robot_service.save_robot_status(robot_status).await {
        Ok(status) => Ok(Json(status)),
        Err(err) => {
            warn!("Failed run path: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    get,
    path = "/hallo-welt",
    responses(
        (status = 200, description = "Hallo Welt"),
    ),
    context_path = "/tibber-developer-test"
)]
pub async fn hallo_welt() -> Json<&'static str> {
    Json("Hallo Welt")
}
