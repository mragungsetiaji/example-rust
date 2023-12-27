use app::user::model::{ UpdatableUser, User };
use app::user::{ request, response };
use crate::middleware::auth;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};

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
    let res = response::UserResponse::from(user, token);
    HttpResponse::Ok().body("users signin")
}

pub async fn signup(
    pool: web::Data<DbPool>,
    form: web::Json<handler::SignupReq>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let user = web::block(move || {
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

    let res = handler::SignupRes::from(user);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn me(req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let user = auth::access_auth_user(&req);

    if let Some(user) = user {
        let user = response::UserResponse::from(user.clone(), user.generate_token());
        Ok(HttpResponse::Ok().json(user));
    } else {
        Ok(HttpResponse::Ok().json({}))
    }
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
    let res = response::UserResponse::from(user, token.to_string());

    Ok(HttpResponse::Ok().json(res))
}