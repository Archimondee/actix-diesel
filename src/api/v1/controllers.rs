use super::{auth_controllers::auth_controllers, user_controllers::user_controllers};
use actix_web::web::{self};

pub fn configure_v1_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("v1")
            .configure(auth_controllers)
            .configure(user_controllers),
    );
}
