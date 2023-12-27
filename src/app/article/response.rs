use crate::app::article::model::Article;
use crate::app::article::tag::model::Tag;
use crate::app::user::model::User;
use serde::{Deserialize, Serialize};

type ArticleCount = i64;

#[derive(Deserialize,Serialize)]
pub struct SingleArticleResponse {
    pub article: ArticleContent,
}

impl SingleArticleResponse {
    pub fn from(article: Article, author: User, tag_list: Vec<Tag>) -> Self {
        Self {
            article::ArticleContent::from(article, user, tag_list),
        }
    }
}

#[derive(Deserialize,Serialize)]
pub struct MultipleArticlesResponse {
    pub articles: Vec<ArticleContent>,
    pub articles_count: ArticleCount,
}

type Info = ((Article, User), Vec<Tag>);
impl MultipleArticlesResponse {
    pub fn from(info: Vec<Info>, user: User, articles_count: ArticleCount) -> Self {
        let articles = inf
            .iter()
            .map(|((article, user), tag_list)| {
                ArticleContent::from(
                    article.to_owned(), 
                    user.clone(), 
                    tag_list.to_owned(),
                )
            })
            .collect();
        Self {
            articlesCount: articles_count,
            articles: articles,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ArticleContent {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub author: AuthorContent,
}

#[derive(Deserialize, Serialize)]
pub struct AuthorContent {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl ArticleContent {
    pub fn from(article: Article, user: User, tag_list: Vec<Tag>) -> Self {
        Self {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: tag_list.iter().map(move |tag| tag.name.clone()).collect(),
            created_at: article.created_at.to_string(),
            updated_at: article.updated_at.to_string(),
        }
    }
}