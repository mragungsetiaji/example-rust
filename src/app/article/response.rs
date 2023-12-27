use crate::app::article::model::Article;
use crate::app::article::tag::model::Tag;
use crate::app::user::model::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct SingleArticleResponse {
    pub article: ArticleContent,
}

impl SingleArticleResponse {
    pub fn from(article: Article, author: User, tag_list: Vec<Tag>) -> Self {
        Self {
            article: ArticleContent {
                slug: article.slug,
                title: article.title,
                description: article.description,
                body: article.body,
                tag_list: tag_list.iter().map(|tag| tag.name.clone()).collect(),
                created_at: article.created_at.to_string(),
                updated_at: article.updated_at.to_string(),
                author: AuthorContent {
                    username: user.username,
                    bio: user.bio,
                    image: user.image,
                    following: false,
                },
            }
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