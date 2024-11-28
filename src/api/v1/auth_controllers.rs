use crate::{
    common::dtos::{create_user_dto::CreateUserDto, login_user_dto::LoginUserDto},
    infrastructure::db::connection::DbPool,
    services::auth_services::AuthService,
};
use actix_web::web;

pub fn auth_controllers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("auth").configure(|cfg_auth| {
        cfg_auth.route(
            "/register",
            web::post().to(
                move |payload: web::Json<CreateUserDto>, pool: web::Data<DbPool>| {
                    AuthService::create_user(payload, pool)
                },
            ),
        );
        cfg_auth.route(
            "/login",
            web::post().to(
                move |payload: web::Json<LoginUserDto>, pool: web::Data<DbPool>| {
                    AuthService::login_user(payload, pool)
                },
            ),
        );
    }));
}
