use std::collections::HashMap;
use std::str;

use actix_web::{Error, get, HttpRequest, HttpResponse, post, Responder, web};
use cqrs_es::AggregateError;
use futures::StreamExt;

use crate::db;

fn process_command(payload_type: &str, aggregate_id: &str, payload: String) -> Result<(), AggregateError> {
    let event_ser = format!("{{\"{}\":{}}}", payload_type, payload);
    let payload = match serde_json::from_str(event_ser.as_str()) {
        Ok(payload) => { payload }
        Err(err) => {
            return Err(AggregateError::TechnicalError(err.to_string()));
        }
    };
    let cqrs = db::cqrs_framework();
    let mut metadata = HashMap::new();
    metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
    cqrs.execute_with_metadata(aggregate_id, payload, metadata)
}

#[get("/lender_group/{query_id}")]
async fn lender_group_query(req: HttpRequest) -> impl Responder {
    let query_id = req.match_info().get("query_id").unwrap_or("").to_string();
    let query_repo = db::QueryRepository::new("lender_group_query", db::db_connection());

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

#[post("/lender_group/{command_type}/{aggregate_id}")]
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

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(lender_group_query);
    config.service(lender_group_command);
}
