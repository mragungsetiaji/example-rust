#[macro_use]
extern crate diesel;

extern crate log;

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
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
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