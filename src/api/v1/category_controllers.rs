use crate::{
    api::health_check::health_check, common::dtos::create_category_dto::CreateCategoryDto,
    infrastructure::db::connection::DbPool, middlewares::auth_middleware::auth_middleware,
    services::category_services::CategoryService,
};
use actix_web::{middleware::from_fn, web};

pub fn category_controllers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("category")
            .configure(|cfg| {
                cfg.route(
                    "/create",
                    web::post().to(
                        move |payload: web::Json<CreateCategoryDto>, pool: web::Data<DbPool>| {
                            CategoryService::create_category(payload, pool)
                        },
                    ),
                );
                cfg.route("/update/{id}", web::put().to(health_check));
                cfg.route("/delete/{id}", web::delete().to(health_check));
                cfg.route("{id}", web::get().to(health_check));
                cfg.route("", web::get().to(health_check));
            })
            .wrap(from_fn(auth_middleware)),
    );
}
