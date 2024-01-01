use super::{ response, service };
use super::service::UnfavoriteService;
use crate::middleware::auth;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

type ArticleIdSlug = String;

pub async fn favorite(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("invaild user");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let article_id = path.into_inner();
    let article_id = Uuid::parse_str(&article_id).expect("invalid article id");

    let (article, profile, tag_list) = service::favorite(
        &conn,
        &service::FavoriteService {
            me: auth_user,
            article_id: article_id,
        },
    );
    let res = response::SingleArticleResponse::from((article, profile, tag_list));

    HttpResponse::Ok().json(res)
}

pub async fn unfavorite(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("invaild user");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let article_id = path.into_inner();
    let article_id = Uuid::parse_str(&article_id).expect("invalid article id");

    let (article, profile, tag_list) = service::unfavorite(
        &conn,
        &UnfavoriteService {
            me: auth_user,
            article_id: article_id,
        },
    );
    let _res = response::SingleArticleResponse::from((article, profile, tag_list));
    
    HttpResponse::Ok().body("unfavorite")
}