use actix_web::web::{self};

use crate::{
    api::health_check::health_check, common::dtos::create_user_dto::CreateUserDto,
    infrastructure::db::connection::DbPool, services::auth_services::AuthService,
};

pub fn configure_v1_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("v1").configure(|cfg| {
        cfg.service(web::scope("auth").configure(|cfg_auth| {
            cfg_auth.route(
                "/register",
                web::post().to(
                    move |payload: web::Json<CreateUserDto>, pool: web::Data<DbPool>| {
                        AuthService::create_user(payload, pool)
                    },
                ),
            );
            cfg_auth.route("/login", web::post().to(health_check));
        }));
    }));
}
