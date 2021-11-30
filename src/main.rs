#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::collections::HashMap;
use std::str;

use futures::StreamExt;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result, web};
use cqrs_es::AggregateError;
use postgres_es::{Connection, GenericQueryRepository, PostgresCqrs};

use crate::aggregate::LenderGroup;
use crate::queries::{LenderGroupQuery, SimpleLoggingQueryProcessor};

mod aggregate;
mod application;
mod commands;
mod events;
mod queries;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/lender_group/{query_id}", web::get().to(lender_group_query))
            .route("/lender_group/{command_type}/{aggregate_id}", web::post().to(lender_group_command))
    })
        .bind("127.0.0.1:3031")?
        .run()
        .await
}

async fn lender_group_command(req: HttpRequest, mut body: web::Payload) -> Result<HttpResponse, Error> {
    let command_type = req.match_info().get("command_type").unwrap_or("");
    let aggregate_id = req.match_info().get("aggregate_id").unwrap_or("");
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let byte_str = str::from_utf8(&bytes).unwrap();
    let payload = byte_str.replace(&['\n', '\t'][..], "");
    let result = match command_type {
        "addLenderGroup" => process_command("AddLenderGroup", aggregate_id, payload),
        "addLender" => process_command("AddLender", aggregate_id, payload),
        "removeLender" => process_command("RemoveLender", aggregate_id, payload),
        _ => Ok(())
    };
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("no content")),
        Err(err) => {
            let err_payload = match &err {
                AggregateError::UserError(e) => serde_json::to_string(e).unwrap(),
                AggregateError::TechnicalError(e)=> e.clone(),
            };
            Err(Error::from(HttpResponse::Ok().body(err_payload)))
        }
    }
}

fn process_command(payload_type: &str, aggregate_id: &str, payload: String) -> Result<(), AggregateError> {
    let event_ser = format!("{{\"{}\":{}}}", payload_type, payload);
    let payload = match serde_json::from_str(event_ser.as_str()) {
        Ok(payload) => { payload }
        Err(err) => {
            return Err(AggregateError::TechnicalError(err.to_string()));
        }
    };
    let cqrs = cqrs_framework();
    let mut metadata = HashMap::new();
    metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
    cqrs.execute_with_metadata(aggregate_id, payload, metadata)
}

async fn lender_group_query(req: HttpRequest) -> impl Responder {
    let query_id = req.match_info().get("query_id").unwrap_or("").to_string();
    let query_repo = LenderGroupQueryRepository::new("lender_group_query", db_connection());

    match query_repo.load(query_id) {
        None => {
            HttpResponse::Ok().body("none")
        }
        Some(query) => {
            let body = serde_json::to_string(&query).unwrap();
            HttpResponse::Ok().content_type("application/json").body(body)
        }
    }
}

type LenderGroupQueryRepository = GenericQueryRepository::<LenderGroupQuery, LenderGroup>;

fn cqrs_framework() -> PostgresCqrs<LenderGroup> {
    let simple_query = SimpleLoggingQueryProcessor {};
    let mut lender_group_query_processor =
        LenderGroupQueryRepository::new("lender_group_query", db_connection());
    lender_group_query_processor.with_error_handler(Box::new(|e| println!("{}", e)));

    postgres_es::postgres_cqrs(db_connection(), vec![Box::new(simple_query), Box::new(lender_group_query_processor)])
}

fn db_connection() -> Connection {
    Connection::new("postgresql://test_user:test_pass@localhost:5432/lender_groups")
}
