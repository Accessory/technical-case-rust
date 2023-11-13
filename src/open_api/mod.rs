use crate::controller;
use crate::models;
use crate::requests;
use crate::robot;
use utoipa::OpenApi;

// OpenApi
#[derive(OpenApi)]
#[openapi(
    info(description = "Process Runner"),
    paths(
        controller::tibber_developer_test::hallo_welt,
        controller::tibber_developer_test::enter_path,
    ),
    tags(
        (name = "controller::tibber_developer_test", description = "The tibber_developer_test controller.")
    ),
    components(schemas(
            models::robot_status::RobotStatus,
            requests::enter_path_request::EnterPathRequest,
            requests::enter_path_request::Commmand,
            robot::map_point::MapPoint,
            robot::map_direction::MapDirection
        ))
)]
pub struct ApiDoc;
