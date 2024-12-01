use crate::{middlewares::auth_middleware::auth_middleware, services::user_services::user_info};
use actix_web::{middleware::from_fn, web};

pub fn user_controllers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("user")
            .configure(|cfg_auth| {
                cfg_auth.route("/info", web::get().to(user_info));
            })
            .wrap(from_fn(auth_middleware)),
    );
}
