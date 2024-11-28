use std::env;

use crate::{
    common::{
        dtos::login_user_dto::LoginUserDto,
        vms::{login_vm::LoginVms, token_vm::Token, user_vm::UserVms},
    },
    core::entities::{auth_entities::Auth, user_entities::User},
    utils::{jwt::create_jwt, log_query, response::ApiError},
};

use bcrypt::verify;
use diesel::{
    query_dsl::methods::FilterDsl, Connection, ExpressionMethods, OptionalExtension, RunQueryDsl,
    SqliteConnection,
};
use r2d2::PooledConnection;

impl LoginUserDto {
    pub fn handle(
        &self,
        conn: &mut PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>,
    ) -> Result<LoginVms, ApiError> {
        use crate::infrastructure::schema::schema::auths::dsl::*;
        use crate::infrastructure::schema::schema::users::dsl::*;

        conn.transaction(|txn_conn| {
            let query = auths.filter(username.eq(&self.username));
            if let Some(auth) =
                log_query(query, || query.first::<Auth>(txn_conn).optional()).unwrap()
            {
                if verify(&self.password, &auth.password).unwrap_or(false) {
                    let query = users.filter(auth_id.eq(&auth.id));
                    if let Some(user) =
                        log_query(query, || query.first::<User>(txn_conn).optional()).unwrap()
                    {
                        let jwt_token = Token {
                            id: auth.id.clone(),
                            username: auth.username.clone(),
                            created_at: auth.created_at.clone(),
                            user_id: user.id.clone(),
                            lastname: user.lastname.clone(),
                            firstname: user.firstname.clone(),
                            email: user.email.clone(),
                        };
                        let token = create_jwt(&jwt_token, &env::var("SECRET_KEY").unwrap());
                        let login = LoginVms {
                            id: auth.id,
                            token,
                            username: auth.username,
                            created_at: auth.created_at,
                            user_info: UserVms {
                                id: user.id,
                                lastname: user.lastname,
                                firstname: user.firstname,
                                email: user.email,
                            },
                        };
                        Ok(login)
                    } else {
                        Err(ApiError {
                            message: "Wrong username or password".to_string(),
                            error: None,
                            status: 200,
                        })
                    }
                } else {
                    Err(ApiError {
                        message: "Wrong username or password".to_string(),
                        error: None,
                        status: 200,
                    })
                }
            } else {
                Err(ApiError {
                    message: "Wrong username or password".to_string(),
                    error: None,
                    status: 200,
                })
            }
        })
    }
}
