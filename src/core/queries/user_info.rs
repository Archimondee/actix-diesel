use diesel::prelude::*;
use diesel::result::Error;
use diesel::JoinOnDsl;
use diesel::{Connection, ExpressionMethods, SqliteConnection};
use r2d2::PooledConnection;

use crate::common::vms::user_vm::UserVms;
use crate::core::aggregator::Aggregator;
use crate::utils::log_query;
use crate::{common::vms::auth_vm::AuthVms, utils::response::ApiError};

pub struct UserInfo {
    pub auth_id_user: String,
}

impl Aggregator<AuthVms> for UserInfo {
    fn handle(
        &self,
        conn: &mut PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>,
    ) -> Result<Option<AuthVms>, ApiError> {
        use crate::infrastructure::schema::schema::auths::dsl::{
            created_at as auth_created, id as auth_id, *,
        };
        use crate::infrastructure::schema::schema::users::dsl::{auth_id as user_auth_id, id, *};
        conn.transaction(|txn_conn| {
            let query = auths
                .filter(auth_id.eq(&self.auth_id_user))
                .inner_join(users.on(user_auth_id.eq(auth_id)));

            let result: Result<
                Option<(
                    String,
                    String,
                    chrono::NaiveDateTime,
                    String,
                    String,
                    String,
                    String,
                )>,
                Error,
            > = log_query(query, || {
                query
                    .select((
                        auth_id,
                        username,
                        auth_created,
                        email,
                        firstname,
                        lastname,
                        id,
                    ))
                    .first::<(
                        String,
                        String,
                        chrono::NaiveDateTime,
                        String,
                        String,
                        String,
                        String,
                    )>(txn_conn)
                    .optional()
            });

            if result.is_ok() {
                let res = result.map(|opt| {
                    opt.map(|e| {
                        let user_info = UserVms {
                            email: e.3,
                            firstname: e.4,
                            lastname: e.5,
                            id: e.6,
                        };

                        AuthVms {
                            id: e.0,
                            username: e.1,
                            created_at: e.2,
                            user_info,
                        }
                    })
                });

                match res {
                    Ok(data) => Ok(data),
                    Err(_) => Err(ApiError {
                        message: "User Not Found".to_string(),
                        error: None,
                        status: 404,
                    }),
                }
            } else {
                Err(ApiError {
                    message: "User Not Found".to_string(),
                    error: None,
                    status: 404,
                })
            }
        })
    }
}
