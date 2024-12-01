use crate::{
    api::health_check::health_check, middlewares::auth_middleware::auth_middleware,
    services::category_services::create_category,
};
use actix_web::{middleware::from_fn, web};

pub fn category_controllers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("category")
            .configure(|cfg| {
                cfg.route("/create", web::post().to(create_category));
                cfg.route("/update/{id}", web::put().to(health_check));
                cfg.route("/delete/{id}", web::delete().to(health_check));
                cfg.route("{id}", web::get().to(health_check));
                cfg.route("", web::get().to(health_check));
            })
            .wrap(from_fn(auth_middleware)),
    );
}
