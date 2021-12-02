use std::collections::HashMap;
use std::str;

use actix_web::{Error, get, HttpRequest, HttpResponse, post, Responder, web};
use cqrs_es::AggregateError;
use futures::StreamExt;

use crate::db;

// Processes the command based on payload type, aggregate_id and payload
// 1. Serializes the event info into a format like:
// 2. Extracts the payload data from the event
fn process_command(payload_type: &str, aggregate_id: &str, payload: String) -> Result<(), AggregateError> {
    let event_ser = format!("{{\"{}\":{}}}", payload_type, payload);
    let payload = match serde_json::from_str(event_ser.as_str()) {
        Ok(payload) => { payload }
        Err(err) => {
            return Err(AggregateError::TechnicalError(err.to_string()));
        }
    };
    // Create a CqrsFramework backed by PostgresStore and using a simple metadata supplier with time of commit
    let cqrs = db::cqrs_framework();
    let mut metadata = HashMap::new();
    // Inserts a key-value pair into the map
    metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
    // Applies a command to an aggregate along with associated metadata
    // Executing a command in this way to make any change to the state of an aggregate
    // An error while processing will result in no events committed and an AggregateError being returned
    // If successful, the events produced will be applied to the configured QueryProcessors
    cqrs.execute_with_metadata(aggregate_id, payload, metadata)
}

// GET route which accepts a query_id (aggregate_id), creates a new QueryRepository
#[get("/lender_group/{query_id}")]
async fn lender_group_query(req: HttpRequest) -> impl Responder {
    let query_id = req.match_info().get("query_id").unwrap_or("").to_string();
    let query_repo = db::QueryRepository::new("lender_group_query", db::db_connection());

    // Loads and deserializes a view based on the query_id (aggregate_id)
    match query_repo.load(query_id) {
        None => {
            HttpResponse::Ok().body("none")
        }
        Some(query) => {
            // Returns and deserializes the query result (current view of the aggregate)
            let body = serde_json::to_string(&query).unwrap();
            HttpResponse::Ok().content_type("application/json").body(body)
        }
    }
}

// POST route which takes in a LenderGroup command and aggregate_id
// 1. Extracts the command_type and aggregate_id from the HttpRequest parameters
// 2. Extracts the payload from the HttpRequest body
// 3. Processes the incoming command
// 4. Returns command result (AggregateError if unsuccessful)
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

// Initializes the routes
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(lender_group_query);
    config.service(lender_group_command);
}
