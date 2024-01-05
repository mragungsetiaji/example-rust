use crate::app::user::model::User;
use crate::constants;
use crate::error::AppError;
use crate::middleware;
use crate::utils::token;
use crate::middleware::state::AppState;
use actix_service::{Service, Transform};
use actix_web::{HttpMessage, App};
use actix_web::{
    dev::ServiceRequest,
    dev::ServiceResponse,
    http::{HeaderName, HeaderValue, Method},
    web::Data,
    Error, HttpRequest, HttpResponse,
};
use diesel::pg::PgConnection;
use futures::future::{ok, Ready};
use futures::Future;
use serde_json::json;
use std::pin::Pin;
use std::task::{Context, Poll};
use uuid::Uuid;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authentication;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body

// <S, B> after a struct or trait name is used to denote generic parameters. 
// S and B are generic parameters representing the type of the next service 
// in the middleware chain and the type of the response body, respectively.
// In the line impl<S, B> Transform<S> for Authentication, Transform<S> is a 
// trait that the Authentication struct is implementing. The S after Transform 
// is a generic parameter for the Transform trait, representing the type of 
// the service that this middleware will be wrapping.
//
// impl<S, B> Transform<S> for Authentication, Transform<S> is a trait that 
// the Authentication struct is implementing. The S after Transform is 
// a generic parameter for the Transform trait, representing the type of 
// the service that this middleware will be wrapping.
//
// In the line type Transform = AuthenticationMiddleware<S>;, AuthenticationMiddleware<S> 
// is a type alias within the Transform implementation. It's saying that, within 
// this implementation, when we refer to Self::Transform, we mean AuthenticationMiddleware<S>.
// The S here is the same S as in impl<S, B> Transform<S> for Authentication, 
// so it represents the type of the next service in the middleware chain.
impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    // This function is used to check if the service is ready to accept a request. 
    // It returns a Poll that indicates whether the service is ready. If the service 
    // is not ready, the current task is scheduled to receive a wakeup when it becomes ready.
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    // This function is called for each request. It takes a mutable reference 
    // to the service (self) and a mutable ServiceRequest (req).
    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        if should_skip_verify(&req) || verify_and_insert_auth_user(&mut req) {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            // This code is creating a Future that, when awaited, will return an HTTP response 
            // with a status of Unauthorized. The async move block is an asynchronous closure 
            // that takes ownership of its environment (in this case, req), and Box::pin is used 
            // to pin this Future in memory.
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(middleware::error::ErrorResponse::from(
                            constants::error_msg::UNAUTHORIZED,
                        ))
                        .into_body(),
                ))
            })
        }
    }
}
    

fn should_skip_verify(req: &ServiceRequest) -> bool {
    if Method::OPTIONS == *req.method() {
        return true;
    }

    for ignore_route in constants::IGNORE_AUTH_ROUTES.iter() {
        if req.path().starts_with(ignore_route) {
            return true;
        }
    }

    false
}

fn find_auth_user(conn: &PgConnection, user_id: Uuid) -> Result<User, AppError> {
    let user = User::find_by_id(&conn, user_id)?;
    Ok(user)
}

fn verify_and_insert_auth_user(req: &mut ServiceRequest) -> bool {
    req.headers_mut().append(
        HeaderName::from_static("content-length"),
        HeaderValue::from_static("true"),
    );

    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        if let Ok(authen_str) = authen_header.to_str() {
            if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                match token::decode(&token) {
                    Ok(token_data) => {
                        let claims = token_data.claims;
                        let user_id = claims.user_id;
                        if let Some(state) = req.app_data::<Data<AppState>>() {
                            let conn = state.get_conn();
                            match conn {
                                Ok(conn) => {
                                    match find_auth_user(&conn, user_id) {
                                        Ok(user) => {
                                            req.head().extensions_mut().insert(user);
                                            return true;
                                        }
                                        Err(_err) => {
                                            return false;
                                        }
                                    };
                                }
                                Err(_err) => {
                                    return false;
                                }
                            }
                        }
                        return false;
                    }
                    _ => {
                        return false;
                    }
                }
            }
        } 
    };
    false
}

pub fn access_auth_user(req: &HttpRequest) -> Result<User, AppError>{
    let head = req.head();
    let extensions = head.extensions();
    let auth_user = extensions.get::<User>();
    let auth_user = auth_user.map(|user| user.to_owned());
    let auth_user = auth_user.ok_or(
        AppError::Unauthorized(json!({
            "error": "Unauthorized"
        }))
    )?;

    Ok(auth_user)
}