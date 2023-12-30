use crate::app::article::model::Article;
use crate::app::tag::model::Tag;
use crate::app::user::model::User;
use serde::{Deserialize, Serialize};

type ArticleCount = i64;

#[derive(Deserialize,Serialize)]
pub struct SingleArticleResponse {
    pub article: ArticleContent,
}

impl std::convert::From<(Article, Profile, Vec<Tag>)> for SingleArticleResponse {
    fn from((article, profile, tag_list): (Article, Profile, Vec<Tag>)) -> Self {
        Self {
            article::ArticleContent {
                slug: article.slug,
                title: article.title,
                description: article.description,
                body: article.body,
                tagList: tag_list
                    .iter()
                    .map(move |tag| tag.name.to_owned())
                    .collect(),
                created_at: article.created_at.to_string(),
                updated_at: article.updated_at.to_string(),
                author: AuthorContent {
                    username: profile.username,
                    bio: profile.bio,
                    image: profile.image,
                    following: profile.following,
                },
            },
        }
    }
}

#[derive(Deserialize,Serialize)]
pub struct MultipleArticlesResponse {
    pub articles: Vec<ArticleContent>,
    pub articles_count: ArticleCount,
}

type Info = ((Article, User), Vec<Tag>);
impl std::convert::From<(Vec<Info>, ArticleCount)> for MultipleArticlesResponse {
    fn from((info, articles_count): (Vec<Info>, ArticleCount)) -> Self {
        let articles = inf
            .iter()
            .map(|((article, user), tag_list)| {
                ArticleContent::DEPRECATED_from(
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
    pub tagList: Vec<String>,
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
    pub fn DEPRECATED_from(article: Article, user: User, tag_list: Vec<Tag>) -> Self {
        Self {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tagList: tag_list.iter().map(move |tag| tag.name.clone()).collect(),
            created_at: article.created_at.to_string(),
            updated_at: article.updated_at.to_string(),
            author: AuthorContent {
                username: user.username,
                bio: user.bio,
                image: user.image,
                following: true,
            },
        }
    }
}
