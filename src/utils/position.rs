use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, ToSchema)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}
