use actix_web::http::StatusCode;
use bcrypt::{hash, DEFAULT_COST};
use diesel::{
    query_dsl::methods::FilterDsl, Connection, ExpressionMethods, OptionalExtension, RunQueryDsl,
    SqliteConnection,
};
use r2d2::PooledConnection;

use crate::{
    common::{
        dtos::create_user_dto::CreateUserDto,
        vms::{auth_vm::AuthVms, user_vm::UserVms},
    },
    core::entities::{auth_entities::Auth, user_entities::User},
    utils::{log_query, response::ApiError},
};

impl CreateUserDto {
    #[allow(dead_code)]
    pub fn handle(
        &self,
        conn: &mut PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>,
    ) -> Result<AuthVms, ApiError> {
        conn.transaction(|txn_conn| {
            use crate::infrastructure::schema::schema::auths::dsl::*;
            use crate::infrastructure::schema::schema::users::dsl::*;

            let query = auths.filter(username.eq(&self.username));
            if let Some(_) = log_query(query, || query.first::<Auth>(txn_conn).optional()).unwrap()
            {
                let response = ApiError {
                    message: "Username already exists".to_string(),
                    status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    error: None,
                };
                return Err(response);
            }

            let query = users.filter(email.eq(&self.email));
            if let Some(_) = log_query(query, || query.first::<User>(txn_conn).optional()).unwrap()
            {
                let response = ApiError {
                    message: "Email already exists".to_string(),
                    status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    error: None,
                };
                return Err(response);
            }

            let hashed_password = hash(&self.password, DEFAULT_COST).unwrap();
            let auth = Auth::new(&self.username, &hashed_password);
            let query = diesel::insert_into(auths).values(&auth);
            let _ = log_query(query, || query.execute(txn_conn));

            let user = User::new(&auth.id, &self.firstname, &self.lastname, &self.email);
            let query = diesel::insert_into(users).values(&user);
            let _ = log_query(query, || query.execute(txn_conn));

            let data = AuthVms {
                id: auth.id,
                username: auth.username,
                created_at: auth.created_at,
                user_info: UserVms {
                    id: user.id,
                    lastname: user.lastname,
                    firstname: user.firstname,
                    email: user.email,
                },
            };

            return Ok(data);
        })
    }
}
