use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum MapDirection {
    #[default]
    North,
    East,
    South,
    West,
}

impl From<&str> for MapDirection {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "north" => MapDirection::North,
            "east" => MapDirection::East,
            "south" => MapDirection::South,
            "west" => MapDirection::West,
            _ => MapDirection::North,
        }
    }
}
