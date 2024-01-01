
use super::service;
use crate::app::profile;
use crate::middleware::auth::access_auth_user;
use crate::AppState;
use actix_web::{ web, HttpRequest, HttpResponse, Responder };

type UsernameSlug = String;

pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> impl Responder {
    
    let me = access_auth_user(&req).expect("invalid user");
    let conn = state
        .pool
        .get()
        .expect("couldnt get db connection from pool");
    let _username = path.into_inner();
    let profile = service::fetch_by_name(
        &conn,
        &service::FetchProfileByName {
            me: me.to_owned(),
            username: _username,
        },
    );

    let res = profile::response::ProfileResponse::from(profile);
    HttpResponse::Ok().json(res)
}

pub async fn follow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> impl Responder {
    
    let user = access_auth_user(&req).expect("invalid user");
    let conn = state
        .pool 
        .get()
        .expect("couldnt get db connection from pool");
    let username = path.into_inner();
    let profile = user.follow(&conn, &username).expect("couldnt follow user");
    HttpResponse::Ok().json(profile)
}

pub async fn unfollow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> impl Responder {

    let user = access_auth_user(&req).expect("invalid user");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let username = path.into_inner();

    let profile = user
        .unfollow(&conn, &username)
        .expect("couldn't unfollow user");
    HttpResponse::Ok().json(profile)
}
