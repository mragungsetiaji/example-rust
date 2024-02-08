#[macro_use]
extern crate diesel;
extern crate actix_web;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod app;
mod constants;
mod error;
mod middleware;
mod routes;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    HttpServer::new(move || {
        let pool = utils::db::establish_connection();
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::cors::cors())
            .wrap(middleware::auth::Authentication)
            .app_data(middleware::state::AppState { pool })
            .configure(routes::api)
    })
    .bind(constants::BIND)?
    .run()
    .await
}