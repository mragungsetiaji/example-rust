use crate::app::user::model::{ UpdatableUser, User };
use crate::app::user::{ request, response };
use crate::middleware::auth;
use crate::AppState;
use actix_web::{web, HttpResponse, HttpRequest};

pub async fn signin(
    state: web::Data<AppState>,
    form: web::Json<request::Signin>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let (user, token) =
        // This line is creating a closure that calls User::signin with conn, 
        // form.user.email, and form.user.password as arguments. The move keyword 
        // is used to take ownership of these variables and move them into 
        // the closure's environment.
        web::block(move || User::signin(&conn, &form.user.email, &form.user.password))
            // asynchronously wait for the blocking operation to complete.
            .await
            // This is error handling. If the web::block operation results in an error, 
            // it is mapped to a different error type. 
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().json(e.to_string())
            // The ? operator is then used to return early if an error occurred.
            })?;

    let res = response::UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn signup(
    state: web::Data<AppState>,
    form: web::Json<request::Signup>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let (user, token) = web::block(move || {
        User::signup(
            &conn,
            &form.user.email,
            &form.user.username,
            &form.user.password,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().json(e.to_string())
    })?;

    let res = response::UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

use crate::error::AppError;

fn happen_err() -> Result<HttpResponse, AppError> {
    Err(AppError::HogeError("this is hoge".to_string()))
}

pub async fn me(_req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let _ = happen_err()?;
    Ok(HttpResponse::Ok().json("hoge"))
}


pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<request::Update>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req).expect("invaild user");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let user = form.user.clone();
    let user = UpdatableUser {
        email: user.email,
        username: user.username,
        password: user.password,
        bio: user.bio,
        image: user.image,
    };
    let user = web::block(move || User::update(&conn, auth_user.id, user))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        })?;
    
    let token = &user.generate_token();
    let res = response::UserResponse::from((user, token.to_string()));
    Ok(HttpResponse::Ok().json(res))
}