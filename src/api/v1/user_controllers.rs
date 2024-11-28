use crate::api::health_check::health_check;
use actix_web::web;

pub fn user_controllers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("user").configure(|cfg_auth| {
        cfg_auth.route("/info", web::post().to(health_check));
    }));
}
