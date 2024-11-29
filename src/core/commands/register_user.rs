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
    core::{
        aggregator::Aggregator,
        entities::{auth_entities::Auth, user_entities::User},
    },
    utils::{log_query, response::ApiError},
};

impl Aggregator<AuthVms> for CreateUserDto {
    fn handle(
        &self,
        conn: &mut PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>,
    ) -> Result<Option<AuthVms>, ApiError> {
        use crate::infrastructure::schema::schema::auths::dsl::*;
        use crate::infrastructure::schema::schema::users::dsl::*;

        conn.transaction(|txn_conn| {
            let query = auths.filter(username.eq(&self.username));
            let username_exists = log_query(query, || query.first::<Auth>(txn_conn).optional())?;

            if username_exists.is_some() {
                return Err(ApiError {
                    message: "Username already exists".to_string(),
                    status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    error: None,
                });
            }

            let query = users.filter(email.eq(&self.email));
            let email_exists = log_query(query, || query.first::<User>(txn_conn).optional())?;

            if email_exists.is_some() {
                return Err(ApiError {
                    message: "Email already exists".to_string(),
                    status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                    error: None,
                });
            }

            let hashed_password = hash(&self.password, DEFAULT_COST).map_err(|err| ApiError {
                message: format!("Password hashing error: {}", err),
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                error: None,
            })?;

            let auth = Auth::new(&self.username, &hashed_password);
            let query = diesel::insert_into(auths).values(&auth);
            log_query(query, || query.execute(txn_conn))?;

            let user = User::new(&auth.id, &self.firstname, &self.lastname, &self.email);
            let query = diesel::insert_into(users).values(&user);
            log_query(query, || query.execute(txn_conn))?;

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

            Ok(Some(data))
        })
    }
}
