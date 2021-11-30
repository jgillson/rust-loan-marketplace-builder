#![forbid(unsafe_code)]
#![deny(clippy::all)]

use actix_web::{App, HttpServer};

mod aggregate;
mod commands;
mod db;
mod events;
mod queries;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // App::new()
        //     .route("/lender_group/{query_id}", web::get().to(lender_group_query))
        //     .route("/lender_group/{command_type}/{aggregate_id}", web::post().to(lender_group_command))
        App::new().configure(routes::init_routes)
    })
        .bind("127.0.0.1:3031")?
        .run()
        .await
}
