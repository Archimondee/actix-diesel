mod api;
mod common;
mod core;
mod infrastructure;
mod middlewares;
mod services;
mod utils;

use crate::utils::swagger::ApiDocV1;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    web::{self, JsonConfig},
    App, HttpServer,
};
use api::{health_check::health_check, v1::controllers::configure_v1_routes};
use log::info;
use simplelog::{Config, SimpleLogger};
use std::time::Duration;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
fn init_logger() {
    SimpleLogger::init(log::LevelFilter::Debug, Config::default())
        .expect("Failed to initialize logger");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    info!("Application is starting up...");
    let governor_conf = GovernorConfigBuilder::default()
        .requests_per_minute(500)
        .period(Duration::from_secs(10))
        .burst_size(50)
        .finish()
        .unwrap();

    let pool = infrastructure::establish_connection();

    HttpServer::new(move || {
        App::new()
            .wrap(utils::logger())
            .wrap(utils::cors())
            .app_data(web::Data::new(pool.clone()))
            .app_data(JsonConfig::default().limit(4096 * 1024))
            .wrap(Governor::new(&governor_conf))
            .service(web::scope("").configure(|cfg| {
                cfg.service(
                    SwaggerUi::new("/swagger/v1/{_:.*}")
                        .url("/api-docs/openapi.json", ApiDocV1::openapi()),
                );
                cfg.route("/health-check", web::get().to(health_check));
                cfg.service(web::scope("api").configure(configure_v1_routes));
            }))
            .default_service(web::route().to(utils::not_found))
    })
    .workers(10)
    .bind("127.0.0.1:5100")?
    .run()
    .await
}
