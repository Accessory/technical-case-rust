mod app_state;
mod configuration;
mod controller;
mod models;
mod open_api;
mod requests;
mod utils;
mod service;

use axum::{response::Redirect, routing::get, Router};
use clap::Parser;
use configuration::Configuration;
use serde_merge::omerge;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use std::{fs::File, io::BufReader, sync::Arc, time::Duration};

use crate::{
    app_state::AppState, configuration::FileConfiguration, controller::tibber_developer_test,
    open_api::ApiDoc, service::robot_service::RobotService,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config: Configuration = init_configuration();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(filter::LevelFilter::DEBUG))
        .init();

    let db_connection_str = config.database_url;

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("Can't connect to database");

    let app_state = Arc::new(AppState { db: pool.clone() });
    let robot_service: Arc<RobotService> = Arc::new(RobotService{state: app_state.clone()});

    // build our application with some routes
    let app = Router::new()
        .nest(
            "/tibber-developer-test/",
            tibber_developer_test::new_router(robot_service),
        )
        .route("/", get(redirect_to_openapi))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // run it with hyper
    let ip = if config.ip == "localhost" {
        "127.0.0.1"
    } else {
        &config.ip
    };
    let addr = format!("{}:{}", ip, config.port);
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(
        &addr
            .parse()
            .expect(&format!("Could not resolve address from {}", addr)),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}

fn init_configuration() -> Configuration {
    let mut configuration = Configuration::parse();
    if configuration.config_file.is_some() {
        let cf = configuration.config_file.as_ref().unwrap();
        let file = File::open(cf).expect("Failed to open the configuration File at: {cf}");
        let reader = BufReader::new(file);
        let config: FileConfiguration =
            serde_yaml::from_reader(reader).expect("Config file could not be parsed");
        configuration = omerge(configuration, config).expect("Failed to merge configs");
    }
    println!("{}", &configuration);
    configuration
}

async fn redirect_to_openapi() -> Redirect {
    Redirect::permanent("/swagger-ui/")
}
