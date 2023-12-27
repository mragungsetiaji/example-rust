use super::model::{Article, NewArticle};
use super::service;
use super::{ request, response };
use crate::app::article::tag::model::{ NewTag, Tag };
use crate::app::user::model::User;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use convert_case::{Case, Casing};
use diesel::Insertable;
use uuid::Uuid;

type ArticleIdSlug = Uuid;

pub async fn index() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("show_articles")
}

pub async fn feed() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("feed of articles")
}

pub async fn show() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("detail_article")
}

pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<request::CreateArticleRequest>,
) -> Result<HttpResponse, HttpResponse> {
    let head = req.head();
    let extensions = head.extensions();
    let auth_user = extensions.get::<User>().expect("invaild user").clone();

    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let (article, tag_list) = service::create(
        &conn,
        &NewArticle {
            author_id: auth_user.id,
            title: form.article.title.clone(),
            slug: form.article.title.to_case(Case::Kebab)
            description: form.article.description.clone(),
            body: form.article.body.clone(),
        },
        &form.article.tagList,
    );

    let res = response::SingleArticleResponse::from(article, auth_user, tag_list);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn update() -> impl Responder {
    // TODO:
    HttpResponse::Ok().body("update_article")
}

pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
) -> impl Responder {
    let head = req.head();
    let extensions = head.extensions();
    let auth_user = extensions.get::<User>().expect("invaild user").clone();

    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let article_id = path.into_inner();
    {
        use crate::schema::articles::dsl::*;
        use diesel::prelude::*;

        diesel::delete(articles.filter(id.eq(article_id)))
            .execute(&conn)
            .expect("Error deleting article");
    }

    HttpResponse::Ok().json({})
}