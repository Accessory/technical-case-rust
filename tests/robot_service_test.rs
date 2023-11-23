#[cfg(test)]
mod robot_service_test {
    use technical_case_rust::{
        requests::enter_path_request::EnterPathRequest, service::robot_service::RobotService,
    };

    #[test]
    fn test_heavy_example() {
        let bytes = include_bytes!("./resources/robotcleanerpathheavy.json");
        let enter_path_request: EnterPathRequest =
            serde_json::from_slice(bytes).expect("Failed parsing robotcleanerpathheavy.json");

        let command_len = enter_path_request.commands.len();
        let result = RobotService::run_path(enter_path_request);
        assert_eq!(result.id, 0);
        assert_eq!(result.commands, command_len as i64);
        assert_eq!(result.result, 993737501);
    }
}
