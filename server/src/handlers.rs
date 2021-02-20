use crate::db;
use actix_web::{HttpResponse, Json, Query, State};
use failure::Error;
use log::debug;

use crate::Server;

pub fn handle_post_csv(server: State<Server>) -> Result<HttpResponse, Error> {
    let logs = Default::default();
    Ok(HttpResponse::Ok().json(api::csv::post::Response(logs)))
}
pub fn handle_post_logs(
    server: State<Server>,
    log: Json<api::logs::post::Request>,
) -> Result<HttpResponse, Error> {
    use crate::model::NewLog;
    use chrono::Utc;

    let log = NewLog {
        user_agent: log.user_agent.clone(),
        response_time: log.response_time,
        timestamp: log.timestamp.unwrap_or_else(|| Utc::now()).naive_utc(),
    };
    let conn = server.pool.get()?;
    db::insert_log(&conn, &log)?;

    debug!("received log: {:?}", log);
    Ok(HttpResponse::Accepted().finish())
}

pub fn handle_get_logs(
    server: State<Server>,
    range: Query<api::logs::get::Query>,
) -> Result<HttpResponse, Error> {
    use chrono::{DateTime, Utc};

    let conn = server.pool.get()?;
    let logs = db::logs(&conn, range.from, range.until)?;
    let logs = logs
        .into_iter()
        .map(|log| api::Log {
            user_agent: log.user_agent,
            response_time: log.response_time,
            timestamp: DateTime::from_utc(log.timestamp, Utc),
        })
        .collect();

    Ok(HttpResponse::Ok().json(api::logs::get::Response(logs)))
}

pub fn handle_get_csv(
    server: State<Server>,
    range: Query<api::csv::get::Query>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", range);

    let csv: Vec<u8> = vec![];
    Ok(HttpResponse::Ok()
        .header("Content-Type", "text/csv")
        .body(csv))
}
