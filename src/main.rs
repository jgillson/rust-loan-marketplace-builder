#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::collections::HashMap;
use std::io::Read;

use cqrs_es::AggregateError;
use iron::{Headers, Iron, IronResult, Request, Response, status};
use postgres_es::{Connection, GenericQueryRepository, PostgresCqrs};
use router::Router;

use crate::aggregate::LenderGroup;
use crate::queries::{LenderGroupQuery, SimpleLoggingQueryProcessor};

mod aggregate;
mod application;
mod commands;
mod events;
mod queries;

fn main() {
    let mut router = Router::new();
    router.get("/lender_group/:query_id", lender_group_query, "lender_group_query");
    router.post("/lender_group/:command_type/:aggregate_id", lender_group_command, "lender_group_command");
    println!("Starting server at http://localhost:3031");
    Iron::new(router).http("localhost:3031").unwrap();
}

pub fn lender_group_command(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let command_type = params.find("command_type").unwrap_or("");
    let aggregate_id = params.find("aggregate_id").unwrap_or("");
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();
    let result = match command_type {
        "addLenderGroup" => process_command("AddLenderGroup", aggregate_id, payload),
        "addLender" => process_command("AddLender", aggregate_id, payload),
        "removeLender" => process_command("RemoveLender", aggregate_id, payload),
        _ => return Ok(Response::with(status::NotFound))
    };
    match result {
        Ok(_) => Ok(Response::with(status::NoContent)),
        Err(err) => {
            let err_payload = match &err {
                AggregateError::UserError(e) => serde_json::to_string(e).unwrap(),
                AggregateError::TechnicalError(e) => e.clone(),
            };
            let mut response = Response::with((status::BadRequest, err_payload));
            response.headers = std_headers();
            Ok(response)
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

pub fn lender_group_query(req: &mut Request) -> IronResult<Response> {
    let query_id = req.extensions.get::<Router>().unwrap().find("query_id").unwrap_or("").to_string();

    let query_repo = GroupQuery::new("group_query", db_connection());
    match query_repo.load(query_id) {
        None => {
            Ok(Response::with(status::NotFound))
        }
        Some(query) => {
            let body = serde_json::to_string(&query).unwrap();
            let mut response = Response::with((status::Ok, body));
            response.headers = std_headers();
            Ok(response)
        }
    }
}

fn std_headers() -> Headers {
    let mut headers = Headers::new();
    let content_type = iron::headers::ContentType::json();
    headers.set(content_type);
    headers
}

type GroupQuery = GenericQueryRepository::<LenderGroupQuery, LenderGroup>;

fn cqrs_framework() -> PostgresCqrs<LenderGroup> {
    let simple_query = SimpleLoggingQueryProcessor {};
    let mut group_query_processor = GroupQuery::new("group_query", db_connection());
    group_query_processor.with_error_handler(Box::new(|e| println!("{}", e)));

    postgres_es::postgres_cqrs(db_connection(), vec![Box::new(simple_query), Box::new(group_query_processor)])
}

fn db_connection() -> Connection {
    Connection::new("postgresql://test_user:test_pass@localhost:5432/lender_groups")
}
