use std::sync::Arc;

use axum::Router;
use hyper::{Body, Request};
use serde_json::json;
use sqlx::PgPool;
use technical_case_rust::{
    app_state::AppState,
    configuration::Configuration,
    controller::tibber_developer_test,
    models::robot_status::RobotStatus,
    open_api::ApiDoc,
    requests::enter_path_request::{Commmand, EnterPathRequest},
    robot::{map_direction::MapDirection, map_point::MapPoint},
    service::robot_service::RobotService,
};
use tower::ServiceExt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[sqlx::test]
async fn hallo_welt(pool: PgPool) {
    let config = Configuration {
        ip: "127.0.0.1".to_owned(),
        port: 55555,
        database_url: "".to_owned(),
        config_file: None,
    };

    let app_state = Arc::new(AppState { db: pool.clone() });
    let robot_service: Arc<RobotService> = Arc::new(RobotService {
        state: app_state.clone(),
    });

    // build our application with some routes
    let app = Router::new()
        .nest(
            "/tibber-developer-test/",
            tibber_developer_test::new_router(robot_service),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // run it with hyper
    let ip = if config.ip == "localhost" {
        "127.0.0.1"
    } else {
        &config.ip
    };
    let addr = format!("{}:{}", ip, config.port);
    tracing::debug!("listening on {}", addr);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/tibber-developer-test/hallo-welt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"\"Hallo Welt\"");
}

#[sqlx::test]
async fn enter_path(pool: PgPool) {
    let config = Configuration {
        ip: "127.0.0.1".to_owned(),
        port: 55555,
        database_url: "".to_owned(),
        config_file: None,
    };

    let app_state = Arc::new(AppState { db: pool.clone() });
    let robot_service: Arc<RobotService> = Arc::new(RobotService {
        state: app_state.clone(),
    });

    // build our application with some routes
    let app = Router::new()
        .nest(
            "/tibber-developer-test/",
            tibber_developer_test::new_router(robot_service),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // run it with hyper
    let ip = if config.ip == "localhost" {
        "127.0.0.1"
    } else {
        &config.ip
    };
    let addr = format!("{}:{}", ip, config.port);
    tracing::debug!("listening on {}", addr);

    let enter_path_request = EnterPathRequest {
        start: MapPoint::new(10, 22),
        commmands: vec![
            Commmand {
                direction: MapDirection::East,
                steps: 2,
            },
            Commmand {
                direction: MapDirection::North,
                steps: 1,
            },
        ],
    };

    let json = json!(enter_path_request);
    let body = serde_json::to_string(&json).unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .header("Content-Type", "application/json")
                .uri("/tibber-developer-test/enter-path")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let robot_status: RobotStatus = serde_json::from_slice(&body).unwrap();
    assert_eq!(robot_status.id, 1);
    assert_eq!(robot_status.commands, 2);
    assert_eq!(robot_status.result, 4);
}
