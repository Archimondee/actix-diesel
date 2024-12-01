use utoipa::{
    openapi::{
        self,
        security::{Http, HttpAuthScheme, SecurityScheme}, ServerBuilder,
    },
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(paths(
        crate::services::auth_services::create_user,
        crate::services::auth_services::login_user,
        crate::services::user_services::user_info,
        crate::services::category_services::create_category
    ),
    
    tags((name = "Library API", description = "Library API from RUST, Actix Web and Diesel ORM")),
    modifiers(&SecurityAddon)
    
)]
pub struct ApiDocV1;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        // NOTE: we can unwrap safely since there already is components registered.
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "Token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
        if openapi.servers.is_none() {
            openapi.servers = Some(Vec::new());
        }

        openapi.servers.as_mut().unwrap().push(
            ServerBuilder::new()
                .url("http://localhost:5100/api/v1")
                .description(Some("Development Server"))
                .build(),
        );
    }
}
