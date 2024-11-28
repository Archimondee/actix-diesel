use actix_web::middleware::Logger;
use diesel::query_builder::QueryFragment;
use diesel::sqlite::Sqlite;
use diesel::{debug_query, QueryResult};
use log::info;
use std::time::Instant;

pub fn logger() -> Logger {
    Logger::new("%a %T \"%r\" %s %b %Dms")
}

pub fn log_query<T, U, F>(query: T, execute_fn: F) -> QueryResult<U>
where
    T: QueryFragment<Sqlite>,
    F: FnOnce() -> QueryResult<U>,
{
    let sql = format!("{}", debug_query::<Sqlite, _>(&query));

    let start = Instant::now();
    let result = execute_fn();
    let duration = start.elapsed();

    let duration_ms = duration.as_micros() as f64 / 1000.0;

    match &result {
        Ok(_) => info!(
            "Query executed successfully in {:.2}ms -> {:?}",
            duration_ms, sql
        ),
        Err(err) => info!(
            "Query failed: {:?}, Duration: {:.2} -> {:?}",
            err, duration_ms, sql
        ),
    }

    result
}
