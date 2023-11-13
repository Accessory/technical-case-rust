use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::robot::{map_direction::MapDirection, map_point::MapPoint};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EnterPathRequest {
    pub start: MapPoint,
    pub commmands: Vec<Commmand>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Commmand {
    pub direction: MapDirection,
    pub steps: usize,
}
