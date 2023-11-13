use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct RobotStatus {
    pub id: i64,
    pub timestamp: NaiveDateTime,
    pub commands: i64,
    pub result: i64,
    pub duration: f64,
}
