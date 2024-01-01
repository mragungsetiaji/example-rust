#[macro_use]
extern crate diesel;
extern crate actix_web;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

mod app;
mod constants;
mod middleware;
mod routes;
mod schema;
mod utils;

pub struct AppState {
    pool: utils::db::DbPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    HttpServer::new(move || {
        let pool = utils::db::establish_connection();
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::cors::cors())
            .wrap(middleware::auth::Authentication)
            .app_data(AppState {pool: pool})
            .service(web::scope("").configure(routes::api))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}