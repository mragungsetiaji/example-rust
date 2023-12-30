use super::model::{Article, NewArticle, UpdateArticle};
use super::service;
use super::{ request, response };
use crate::middleware::auth;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;

type ArticleIdSlug = Uuid;

#[derive(Deserialize)]
pub struct ArticlesListQueryParameter {
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn index(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<ArticlesListQueryParameter>,
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("invaild user");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    
    /// Calculates the offset and limit for pagination.
    ///
    /// The `offset` is the number of items to skip before starting to return items.
    /// It defaults to 0 if `params.offset` is `None`, but is capped at a maximum of 100.
    let offset = std::cmp::max(params.offset.to_owned().unwrap_or(0), 100);
    /// The `limit` is the maximum number of items to return.
    /// It defaults to 20 if `params.limit` is `None`.
    let limit = params.limit.unwrap_or(20);

    let (articles_list, articles_count) = {
        let articles_list = service::fetch_articles_list(
            &conn,
            &service::FetchArticlesList {
                tag: params.tag.clone(),
                author: params.author.clone(),
                favorited: params.favorited.clone(),
                offset,
                limit,
            },
        );
        
        let articles_count = service::fetch_articles_count(&conn);

        (articles_list, articles_count)
    };
    let res = response::MultipleArticlesResponse::from((
        articles_list,
        articles_count,
    ));

    HttpResponse::Ok().json(res)
}

#[derive(Deserialize)]
pub struct FeedQueryParameter {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn feed(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<FeedQueryParameter>,
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("invaild user");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let offset = std::cmp::min(params.offset.to_owned().unwrap_or(0), 100);
    let limit = params.limit.unwrap_or(20);
    let (articles_list, articles_count) = service::fetch_following_articles(
        &conn,
        &service::FetchFollowingArticles {
            me: auth_user,
            offset,
            limit,
        },
    );

    let res = response::MultipleArticlesResponse::from((
        articles_list,
        articles_count,
    ));
    HttpResponse::Ok().json(res)
}

pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web:;Path<ArticleIdSlug>,
) -> impl Responder {
    let auth_user = auth::access_auth_user(&req).expect("invaild user");
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let article_id = path.into_inner();
    let (article, profile, tag_list) = service::fetch_article(
        &conn,
        &service::FetchArticle {
            article_id: article_id,
            me: auth_user,
        },
    );
    
    let res = response::SingleArticleResponse::from((
        article,
        profile,
        tag_list,
    ));
    HttpResponse::Ok().json(res)
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
    let (article, profile, tag_list) = service::create(
        &conn,
        &service::CreateArticleService {
            author_id: auth_user.id,
            title: form.article.title.clone(),
            slug: Article::convert_title_to_slug(&form.article.title),
            description: form.article.description.clone(),
            body: form.article.body.clone(),
            tag_list: form.article.tagList.to_owned(),
            me: auth_user,
        },
    );

    let res = response::SingleArticleResponse::from((article, profile, tag_list));
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
    
    let article_slug = &form
        .article
        .title 
        .as_ref()
        .map(|_title| Article::convert_title_to_slug(title));
    }

    let (article, profile, tag_list) = service::update_article(
        &conn,
        &service::UpdateArticleService {
            me: auth_user,
            article_id,
            slug: article_slug.to_owned(),
            title: form.article.title.clone(),
            description: form.article.description.clone(),
            body: form.article.body.clone(),
        },
    )
    let res = response::SingleArticleResponse::from((article, profile, tag_list));
    
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