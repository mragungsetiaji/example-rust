use crate::app::article::model::Article;
use crate::app::profile::model::Profile;
use crate::app::tag::model::Tag;
use serde::{Deserialize, Serialize};
use std::convert::From;
type ArticleCount = i64;

#[derive(Deserialize, Serialize)]
pub struct SingleArticleResponse {
    pub article: ArticleContent,
}

impl From<(Article, Profile, Vec<Tag>)> for SingleArticleResponse {
    fn from((article, profile, tag_list): (Article, Profile, Vec<Tag>)) -> Self {
        Self {
            article: ArticleContent {
                slug: article.slug,
                title: article.title,
                description: article.description,
                body: article.body,
                tag_list: tag_list
                    .iter()
                    .map(move |tag| tag.name.to_owned())
                    .collect(),
                created_at: article.created_at.to_string(),
                updated_at: article.updated_at.to_string(),
                favorited: false,
                favorites_count: 0,
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

#[derive(Deserialize, Serialize)]
pub struct MultipleArticlesResponse {
    pub articles: Vec<ArticleContent>,
    pub articles_count: ArticleCount,
}

type IsFavorited = bool;
type FavoritedCount = i64;
type ArticlesCount = i64;
type Inner = (((Article, Profile, IsFavorited), FavoritedCount), Vec<Tag>);
type ArticlesList = Vec<Inner>;
type Item = (ArticlesList, ArticlesCount);

impl From<Item> for MultipleArticlesResponse {
    fn from((list, articles_count): (Vec<Inner>, ArticleCount)) -> Self {
        let articles = list
            .iter()
            .map(
                |(((article, profile, is_favorited), favorited_count), tags_list)| {
                    ArticleContent::from((
                        article.to_owned(), 
                        profile.to_owned(), 
                        is_favorited.to_owned(),
                        favorited_count.to_owned(),
                        tags_list.to_owned(),
                    ))
                },
            )
            .collect();
        Self {
            articles_count: articles_count,
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
    pub favorited: bool,
    pub favorites_count: i64,
    pub author: AuthorContent,
}

impl From<(Article, Profile, IsFavorited, FavoritedCount, Vec<Tag>)> for ArticleContent {
    fn from(
        (article, profile, is_favorited, favorited_count, tags_list): (
            Article,
            Profile,
            IsFavorited,
            FavoritedCount,
            Vec<Tag>,
        ),
    ) -> Self {
        Self {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: tags_list
                .iter()
                .map(move |tag| tag.name.clone())
                .collect(),
            created_at: article.created_at.to_string(),
            updated_at: article.updated_at.to_string(),
            favorited: is_favorited,
            favorites_count: favorited_count,
            author: AuthorContent {
                username: profile.username,
                bio: profile.bio,
                image: profile.image,
                following: profile.following,
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AuthorContent {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
