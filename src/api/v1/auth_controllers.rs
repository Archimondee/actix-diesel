use crate::services::auth_services::{create_user, login_user};
use actix_web::web;

pub fn auth_controllers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("auth").configure(|cfg_auth| {
        cfg_auth.route("/register", web::post().to(create_user));
        cfg_auth.route("/login", web::post().to(login_user));
    }));
}
