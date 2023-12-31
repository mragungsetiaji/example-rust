use super::model::Comment;
use super::{request, response, service};
use crate::middleware::auth;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

type ArticleIdSlug = String;
type CommentIdSlug = String;

pub async fn index(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req).expect("invaild user");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let lit = service::fetch_comments_list(&conn, &auth_user);
    let res = response::MultipleCommentsResponse::from(lit);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::CreateCommentRequest>,
) -> Result<HttpResponse, HttpResponse> {
    let auth_user = auth::access_auth_user(&req).expect("invaild user");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let article_id = path.into_inner();
    let article_id = Uuid::parse_str(&article_id).expect("invalid article id");
    let (comment, profile) = service::create(
        &conn,
        &service::CreateCommentService {
            body: form.comment.body.to_owned(),
            article_id: article_id,
            author: auth_user,
        },
    );

    let res = response::SingleCommentResponse::from((comment, profile));  
    Ok(HttpResponse::Ok().json(res))
}

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(ArticleIdSlug, CommentIdSlug)>,    
) -> Result<HttpResponse, HttpResponse> {
    let _auth_user = auth::access_auth_user(&req).expect("invaild user");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let (article_id, comment_id) = path.into_inner();
    let _article_id = Uuid::parse_str(&article_id).expect("invalid article id");
    let comment_id = Uuid::parse_str(&comment_id).expect("invalid comment id");

    Comment::delete(&conn, &comment_id);

    Ok(HttpResponse::Ok().json("OK"))
}