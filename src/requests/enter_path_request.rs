use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utils::position::Position;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EnterPathRequest {
    pub start: Position,
    pub commands: Vec<Command>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Command {
    pub direction: String,
    pub steps: usize,
}
