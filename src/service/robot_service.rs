use std::sync::Arc;

use chrono::Utc;
use tokio::time::Instant;

use crate::{
    app_state::AppState,
    models::robot_status::RobotStatus,
    requests::enter_path_request::EnterPathRequest,
    utils::{line::Line, position::Position},
};

pub struct RobotService {
    pub state: Arc<AppState>,
}

impl RobotService {
    pub async fn save_robot_status(
        &self,
        mut robot_status: RobotStatus,
    ) -> Result<RobotStatus, sqlx::Error> {
        let next_id: i64 = sqlx::query_scalar("Select nextval('jobs_id_seq')")
            .fetch_one(&self.state.db)
            .await?;
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

    pub fn run_path(request: EnterPathRequest) -> RobotStatus {
        let start = Instant::now();

        let mut current_position = request.start;

        let mut sum = 1;
        let to_check = request.commands.len() - 1;

        let mut lines: Vec<Line> = Vec::new();

        for (i, command) in request.commands.iter().enumerate() {
            sum += command.steps as i64;
            match command.direction.to_lowercase().as_str() {
                "north" => {
                    lines.push(Line {
                        start: current_position,
                        end: Position {
                            x: current_position.x,
                            y: current_position.y
                                + (command.steps as i64 - if to_check == i { 0 } else { 1 }),
                        },
                    });
                    current_position.y += command.steps as i64;
                }
                "south" => {
                    lines.push(Line {
                        start: current_position,
                        end: Position {
                            x: current_position.x,
                            y: current_position.y
                                - (command.steps as i64 - if to_check == i { 0 } else { 1 }),
                        },
                    });
                    current_position.y -= command.steps as i64;
                }
                "west" => {
                    lines.push(Line {
                        start: current_position,
                        end: Position {
                            x: current_position.x
                                - (command.steps as i64 - if to_check == i { 0 } else { 1 }),
                            y: current_position.y,
                        },
                    });
                    current_position.x -= command.steps as i64;
                }
                "east" => {
                    lines.push(Line {
                        start: current_position,
                        end: Position {
                            x: current_position.x
                                + (command.steps as i64 - if to_check == i { 0 } else { 1 }),
                            y: current_position.y,
                        },
                    });
                    current_position.x += command.steps as i64;
                }
                _ => {}
            }
        }

        let mut intersection = 0;

        for (i, l1) in lines.iter().enumerate() {
            for l2 in lines.iter().skip(i + 1) {
                intersection += l1.intersections(l2);
            }
        }

        let elapsed = start.elapsed();

        RobotStatus {
            id: 0,
            timestamp: Utc::now().naive_utc(),
            commands: request.commands.len() as i64,
            result: sum - intersection,
            duration: elapsed.as_secs_f64(),
        }
    }
}
