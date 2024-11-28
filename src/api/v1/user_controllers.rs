use crate::{
    infrastructure::db::connection::DbPool, middlewares::auth_middleware::auth_middleware,
    services::user_services::UserService,
};
use actix_web::{middleware::from_fn, web, HttpRequest};

pub fn user_controllers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("user")
            .configure(|cfg_auth| {
                cfg_auth.route(
                    "/info",
                    web::get().to(move |req: HttpRequest, pool: web::Data<DbPool>| {
                        UserService::user_info(req, pool)
                    }),
                );
            })
            .wrap(from_fn(auth_middleware)),
    );
}
