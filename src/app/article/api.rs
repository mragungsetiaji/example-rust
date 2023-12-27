use super::model::{Article, NewArticle, UpdateArticle};
use super::service;
use super::{ request, response };
use crate::app::article::tag::model::Tag;
use crate::app::user::model::User;
use crate::middleware::auth;
use crate::schema::users;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::associations::HasTable;
use uuid::Uuid;

type ArticleIdSlug = Uuid;

pub async fn index(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("invaild user");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let offset = 0;
    let limit = 20;

    let (articles_list, articles_count) = {
        use crate::schema::articles::dsl::*;
        use diesel::prelude::*;

        let articles_and_user_list = articles
            .inner_join(users::table)
            .offset(offset)
            .limit(limit)
            .get_results::<(Article, User)>(&conn)
            .expect("Error loading articles");

        let articles_list = article_and_user_list
            .clone()
            .into_iter()
            .map(|(article, _)| article)
            .collect::<Vec<_>>();
        let tags_list = Tag::belonging_to(&articles_list)
            .load::<Tag>(&conn)
            .expect("Error loading tags");
        let tags_list = tags_list
            .grouped_by(&articles_list)
        let articles_list = article_and_user_list 
            .into_iter()
            .zip(tags_list)
            .collect::<Vec<_>>();
        let articles_count = articles
            .select(diesel::dsl::count(id))
            .first::<i64>(&conn)
            .expect("Error loading articles count");

        (articles_list, articles_count)
    };
    let res = response::MultipleArticlesResponse::from(
        articles_list,
        articles_count,
    );

    HttpResponse::Ok().json(res)
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
    let auth_user = auth::access_auth_user(&req).expect("invaild user");

    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let (article, tag_list) = service::create(
        &conn,
        &NewArticle {
            author_id: auth_user.id,
            title: form.article.title.clone(),
            slug: Article::convert_title_to_slug(&form.article.title),
            description: form.article.description.clone(),
            body: form.article.body.clone(),
        },
        &form.article.tagList,
    );

    let res = response::SingleArticleResponse::from(article, auth_user.clone(), tag_list);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<ArticleIdSlug>,
    form: web::Json<request::UpdateArticleRequest>,
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("invaild user");
    
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let article_id = path.into_inner();
    let (article, tag_list) = {
        let new_slug = &form
            .article
            .title
            .as_ref()
            .map(|_title| Article::convert_title_to_slug(_title))
        let article = Article::update(
            &conn,
            &article_id,
            &UpdateArticle {
                slug: new_slug.to_owned(),
                title: form.article.title.clone(),
                description: form.article.description.clone(),
                body: form.article.body.clone(),
            },
        );
        let tag_list = vec![];
        (article, tag_list)
    }
    let res = response::SingleArticleResponse::from(article, auth_user, tag_list);
    
    HttpResponse::Ok().json(res)
}

pub async fn delete(
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
    {
        use crate::schema::articles::dsl::*;
        use diesel::prelude::*;

        diesel::delete(articles.filter(id.eq(article_id)))
            .execute(&conn)
            .expect("Error deleting article");
    }

    HttpResponse::Ok().json({})
}