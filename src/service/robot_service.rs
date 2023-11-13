use std::{collections::HashSet, sync::Arc};

use chrono::Utc;
use tokio::time::Instant;

use crate::{
    app_state::AppState,
    models::robot_status::RobotStatus,
    requests::enter_path_request::EnterPathRequest,
    robot::{
        map_direction::MapDirection,
        map_point::{MapPoint, MapWalker},
    },
};

pub struct RobotService {
    pub state: Arc<AppState>,
}

impl RobotService {
    pub async fn save_robot_status(&self, mut robot_status: RobotStatus) -> Result<RobotStatus, sqlx::Error> {
        let next_id:i64 = sqlx::query_scalar("Select nextval('jobs_id_seq')").fetch_one(&self.state.db).await?;
        robot_status.id = next_id;


        sqlx::query("INSERT INTO public.jobs(id, \"timestamp\", commands, result, duration) VALUES ($1,$2, $3, $4, $5);")
        .bind(robot_status.id)
        .bind(robot_status.timestamp)
        .bind(robot_status.commands)
        .bind(robot_status.result)
        .bind(robot_status.duration)
        .execute(&self.state.db)
        .await?;

        Ok(robot_status)
    }

    pub fn run_path(req: EnterPathRequest) -> RobotStatus {
        let start = Instant::now();

        let mut map: HashSet<MapPoint> = HashSet::new();
        let mut walker = MapWalker {
            direction: MapDirection::North,
            position: MapPoint::new(req.start.x, req.start.y),
        };

        map.insert(walker.position);
        for command in req.commmands.iter() {
            for _ in 0..command.steps {
                walker.position.move_by_direction(&command.direction);
                map.insert(walker.position);
            }
        }

        let elapsed = start.elapsed();

        RobotStatus {
            id: 0,
            timestamp: Utc::now().naive_utc(),
            commands: req.commmands.len() as i64,
            result: map.len() as i64,
            duration: elapsed.as_secs_f64(),
        }
    }
}
