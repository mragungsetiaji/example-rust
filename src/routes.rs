use crate::app;
use actix_web::web;
use actix_web::web::{delete, get, post, put};

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/tags").route("", get().to(app::article::tag::api::index)))
            .service(
                web::scope("/users")
                    .route("/login", post().to(app::user::api::signin))
                    .route("", post().to(app::user::api::signup)),
            )
            .service(
                web::scope("/user")
                    .route("", get().to(app::user::api::me))
                    .route("", put().to(app::user::api::update)),
            )
            .service(
                web::scope("/profiles")
                    .route("/{username}", get().to(app::profile::api::show))
                    .route("/{username}/follow", post().to(app::profile::api::follow))
                    .route(
                        "/{username}/follow",
                        delete().to(app::profile::api::unfollow),
                    ),
            )
            .service(
                web::scope("/articles/{id}/comments")
                    .route("", get().to(app::article::comment::api::index))
                    .route("", post().to(app::article::comment::api::create))
                    .route("", delete().to(app::article::comment::api::delete)),
            )
            .service(
                web::scope("/articles/{id}/favorites")
                    .route("", post().to(app::article::favorite::api::favorite))
                    .route("", delete().to(app::article::favorite::api::unfavorite)),
            )
            .service(
                web::scope("/articles")
                    .route("", get().to(app::article::api::index))
                    .route("/feed", get().to(app::article::api::feed))
                    .route("/{id}", get().to(app::article::api::show))
                    .route("", post().to(app::article::api::create))
                    .route("/{id}", put().to(app::article::api::update))
                    .route("", delete().to(app::article::api::delete)),
            ),
    );
}