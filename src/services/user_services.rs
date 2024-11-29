use crate::core::aggregator::Aggregator;
use crate::core::queries::user_info::UserInfo;
use crate::utils::check_connection::check_connection;
use crate::utils::jwt::Claims;
use crate::utils::status_response::status_response;
use crate::{infrastructure::db::connection::DbPool, utils::response::create_response};
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse};

pub struct UserService {}

impl UserService {
    pub async fn user_info(
        req: HttpRequest,
        pool: web::Data<DbPool>,
    ) -> Result<HttpResponse, Error> {
        let mut conn = check_connection(pool);

        if let Some(claims) = req.extensions().get::<Claims>() {
            let user_info = UserInfo {
                auth_id_user: claims.id.clone(),
            };

            match user_info.handle(&mut conn) {
                Ok(user) => {
                    let response =
                        create_response("Success", StatusCode::OK.as_u16(), Some(user), None, None);
                    Ok(status_response(StatusCode::OK.as_u16(), response).unwrap())
                }
                Err(e) => {
                    let response = create_response(&e.message, e.status, Some({}), None, e.error);
                    Ok(status_response(e.status, response).unwrap())
                }
            }
        } else {
            let response = create_response("Unauthorized", 401, Some({}), None, None);
            Ok(status_response(401, response).unwrap())
        }
    }
}
