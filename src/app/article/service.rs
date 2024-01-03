use crate::app::article::model::{Article, NewArticle, UpdateArticle};
use crate::app::favorite;
use crate::app::favorite::model::FavoriteInfo;
use crate::app::follow::model::Follow;
use crate::app::profile;
use crate::app::profile::model::Profile;
use crate::app::profile::service::FetchProfileById;
use crate::app::tag::model::{NewTag, Tag};
use crate::app::user::model::User;
use crate::schema::articles::dsl::*;
use crate::schema::{articles, favorites, tags, users};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub struct CreateArticleSerivce {
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
    pub me: User,
}
pub fn create(
    conn: &PgConnection, 
    params: &CreateArticleSerivce
) -> (Article, Profile, FavoriteInfo, Vec<Tag>) {
    let article = Article::create(
        &conn,
        &NewArticle {
            author_id: params.author_id,
            slug: params.slug.to_owned(),
            title: params.title.to_owned(),
            description: params.description.to_owned(),
            body: params.body.to_owned(),
        },
    );
    let tag_list = create_tag_list(&conn, &params.tag_list, &article);
    let profile = profile::service::fetch_profile_by_id(
        &conn,
        &FetchProfileById {
            me: params.me.to_owned(),
            id: article.author_id,
        },
    );
    let favorites_count = favorite::service::fetch_favorites_count_by_article_id(conn, article.id);
    let favorited_article_ids = favorite::service::fetch_favorited_article_ids_by_user_id(conn, params.me.id);
    let is_favorited = favorited_article_ids
        .to_owned()
        .into_iter()
        .any(|_id| _id == article.id);
    let favorite_info = FavoriteInfo {
        is_favorited,
        favorites_count,
    };
    (article, profile, favorite_info, tag_list)
}

fn create_tag_list(
    conn: &PgConnection,
    tag_list: &Option<Vec<String>>,
    article: &Article,
) -> Vec<Tag> {
    tag_list
        .as_ref()
        .map(|tag_list| {
            let records = tag_list
                .iter()
                .map(|tag| NewTag {
                    name: &tag,
                    article_id: &article.id,
                })
                .collect();
            Tag::create_list(&conn, records)
        })
        .unwrap_or(vec![])
}

pub struct FetchArticlesList {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: i64,
    pub limit: i64,
    pub me: User,
}

type ArticlesCount = i64;
type ArticlesListInner = (Article, Profile, FavoriteInfo);
type ArticlesList = Vec<(ArticlesListInner, Vec<Tag>)>;
pub fn fetch_articles_list(
    conn: &PgConnection,
    params: FetchArticlesList,
) -> (ArticlesList, ArticlesCount) {
    use diesel::prelude::*;
    let query = || {
        let mut query = articles::table.inner_join(users::table).into_boxed();

        if let Some(tag_name) = &params.tag {
            let tagged_article_ids = tags::table
                .filter(tags::name.eq(tag_name))
                .select(tags::article_id)
                .load::<Uuid>(conn)
                .expect("could not fetch tagged article ids.");
            query = query.filter(articles::id.eq_any(tagged_article_ids));
        }

        if let Some(author_name) = &params.author {
            let article_ids_by_author = users::table
                .inner_join(articles::table)
                .filter(users::username.eq(author_name))
                .select(articles::id)
                .load::<Uuid>(conn)
                .expect("could not fetch authors id.");
            query = query.filter(articles::id.eq_any(article_ids_by_author));
        }

        if let Some(favorited_username) = &params.favorited {
            let favorited_article_ids = favorites::table
                .inner_join(users::table)
                .filter(users::username.eq(favorited_username))
                .select(favorites::article_id)
                .load::<Uuid>(conn)
                .expect("could not fetch favorited articles id.");
            query = query.filter(articles::id.eq_any(favorited_article_ids));
        }

        query
    };

    let articles_count = query()
        .select(diesel::dsl::count(articles::id))
        .first::<i64>(conn)
        .expect("couldn't fetch articles count.");

    let articles_list = {
        let article_and_user_list = query()
            .offset(params.offset)
            .limit(params.limit)
            .load::<(Article, User)>(conn)
            .expect("couldn't fetch articles list.");

        let tags_list = {
            let articles_list = article_and_user_list
                .clone()
                .into_iter()
                .map(|(article, _)| article)
                .collect::<Vec<_>>();

            let tags_list = Tag::belonging_to(&articles_list)
                .load::<Tag>(conn)
                .expect("could not fetch tags list.");

            let tags_list: Vec<Vec<Tag>> = tags_list.grouped_by(&articles_list);
            tags_list
        };

        let article_ids_list = article_and_user_list
            .clone()
            .into_iter()
            .map(|(article, _)| article.id)
            .collect::<Vec<_>>();

        let favorites_count_list = article_ids_list
            .into_iter()
            .map(|article_id| {
                let favorites_count = favorites::table
                    .filter(favorites::article_id.eq_all(article_id))
                    .select(diesel::dsl::count(favorites::created_at))
                    .first::<i64>(conn)
                    .expect("could not favorites count list.");
                favorites_count
            })
            .collect::<Vec<_>>();

        let favorited_article_ids = favorites::table
            .filter(favorites::user_id.eq(params.me.id))
            .select(favorites::user_id)
            .get_results::<Uuid>(conn)
            .expect("could not find favorited articles ids.");

        let is_favorited_by_me = |article: &Article| {
            favorited_article_ids
                .to_owned()
                .into_iter()
                .any(|_id| _id == article.id)
        };

        let article_and_profile_list = {
            let user_ids_list = article_and_user_list
                .clone() // TODO: avoid clone
                .into_iter()
                .map(|(_, user)| user.id)
                .collect::<Vec<_>>();

            let follows_list = follows::table
                .filter(follows::follower_id.eq(params.me.id))
                .filter(follows::followee_id.eq_any(user_ids_list))
                .get_results::<Follow>(conn)
                .expect("could not fetch follow.");

            let follows_list = follows_list.into_iter();
            let article_and_profile_list = article_and_user_list
                .into_iter()
                .map(|(article, user)| {
                    let following = follows_list.clone().any(|item| item.followee_id == user.id);
                    let profile = Profile {
                        username: user.username,
                        bio: user.bio,
                        image: user.image,
                        following: following.to_owned(),
                    };
                    let favorited = is_favorited_by_me(&article);
                    (article, profile, favorited)
                })
                .zip(favorites_count_list)
                .map(|((article, profile, is_favorited), favorites_count)| {
                    (article, profile, FavoriteInfo { is_favorited, favorites_count })
                })
                .collect::<Vec<_>>();

            article_and_profile_list
        };

        let articles_list = article_and_profile_list
            .into_iter()
            .zip(tags_list)
            .collect::<Vec<_>>();

        articles_list
    };

    (articles_list, articles_count)
}

pub struct FetchArticle {
    pub article_id: Uuid,
    pub me: User,
}
pub fn fetch_article(
    conn: &PgConnection, 
    params: &FetchArticle
) -> (Article, Profile, FavoriteInfo, Vec<Tag>) {
    use diesel::prelude::*;
    let FetchArticle { article_id, me } = params;
    let (article, author) = articles
        .inner_join(users::table)
        .filter(articles::id.eq(article_id))
        .get_result::<(Article, User)>(conn)
        .expect("could not fetch article by id.");

    let profile = profile::service::fetch_profile_by_id(
        &conn,
        &FetchProfileById {
            me: me.to_owned(),
            id: author.id,
        },
    );

    let favorited_article_ids =
        favorite::service::fetch_favorited_article_ids_by_user_id(conn, params.me.id);

    let is_favorited = favorited_article_ids
        .to_owned()
        .into_iter()
        .any(|_id| _id == article.id);

    let favorite_info = FavoriteInfo {
        is_favorited,
        favorites_count: favorite::service::fetch_favorites_count_by_article_id(conn, article.id),
    };

    let tags_list = Tag::belonging_to(&article)
        .load::<Tag>(conn)
        .expect("could not fetch tags list.");

    (article, profile, favorite_info, tags_list)
}

use crate::schema::follows;
use crate::schema::follows::dsl::*;
pub struct FetchFollowedArticlesSerivce {
    pub me: User,
    pub offset: i64,
    pub limit: i64,
}
pub fn fetch_following_articles(
    conn: &PgConnection,
    params: &FetchFollowedArticlesSerivce,
) -> (ArticlesList, ArticlesCount) {
    let query = {
        let following_user_ids = follows
            .filter(follows::follower_id.eq(params.me.id))
            .select(follows::followee_id)
            .get_results::<Uuid>(conn)
            .expect("could not fetch following uesrs.");

        articles.filter(articles::author_id.eq_any(following_user_ids))
    };

    let articles_list = {
        let article_and_user_list = query
            .to_owned()
            .inner_join(users::table)
            .limit(params.limit)
            .offset(params.offset)
            .order(articles::created_at.desc())
            .get_results::<(Article, User)>(conn)
            .expect("could not fetch following articles.");

        let tags_list = {
            let articles_list = article_and_user_list
                .clone() // TODO: avoid clone
                .into_iter()
                .map(|(article, _)| article)
                .collect::<Vec<_>>();

            let tags_list = Tag::belonging_to(&articles_list)
                .load::<Tag>(conn)
                .expect("could not fetch tags list.");

            let tags_list: Vec<Vec<Tag>> = tags_list.grouped_by(&articles_list);
            tags_list
        };

        let article_and_profile_list = {
            let user_ids_list = article_and_user_list
                .clone() // TODO: avoid clone
                .into_iter()
                .map(|(_, user)| user.id)
                .collect::<Vec<_>>();

            let follows_list = follows::table
                .filter(follows::follower_id.eq(params.me.id))
                .filter(follows::followee_id.eq_any(user_ids_list))
                .get_results::<Follow>(conn)
                .expect("could not fetch follow.");

            let article_ids_list = article_and_user_list
                .clone()
                .into_iter()
                .map(|(article, _)| article.id)
                .collect::<Vec<_>>();

            let favorites_count_list = article_ids_list
                .into_iter()
                .map(|article_id| {
                    let favorites_count = favorites::table
                        .filter(favorites::article_id.eq_all(article_id))
                        .select(diesel::dsl::count(favorites::created_at))
                        .first::<i64>(conn)
                        .expect("could not favorites count list.");
                    favorites_count
                })
                .collect::<Vec<_>>();

            let favorited_article_ids = favorites::table
                .filter(favorites::user_id.eq(params.me.id))
                .select(favorites::user_id)
                .get_results::<Uuid>(conn)
                .expect("could not find favorited articles ids.");

            let is_favorited_by_me = |article: &Article| {
                favorited_article_ids
                    .to_owned()
                    .into_iter()
                    .any(|_id| _id == article.id)
            };

            let follows_list = follows_list.into_iter();
            let article_and_profile_list = article_and_user_list
                .into_iter()
                .map(|(article, user)| {
                    let following = follows_list.clone().any(|item| item.followee_id == user.id);
                    let profile = Profile {
                        username: user.username,
                        bio: user.bio,
                        image: user.image,
                        following: following.to_owned(),
                    };
                    let is_favorited = is_favorited_by_me(&article);
                    (article, profile, is_favorited)
                })
                .zip(favorites_count_list)
                .map(|((article, profile, is_favorited), favorites_count)| {
                    (article, profile, FavoriteInfo { is_favorited, favorites_count })
                })
                .collect::<Vec<_>>();

            article_and_profile_list
        };

        let list = article_and_profile_list
            .into_iter()
            .zip(tags_list)
            .collect::<Vec<_>>();

        list
    };

    let articles_count = query
        .select(diesel::dsl::count(articles::id))
        .first::<i64>(conn)
        .expect("couldn't fetch articles count.");

    (articles_list, articles_count)
}

pub struct UpdateArticleService {
    pub me: User,
    pub article_id: Uuid,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}
pub fn update_article(
    conn: &PgConnection,
    params: &UpdateArticleService,
) -> (Article, Profile, FavoriteInfo, Vec<Tag>) {
    let article = Article::update(
        &conn,
        &params.article_id,
        &UpdateArticle {
            slug: params.slug.to_owned(),
            title: params.title.to_owned(),
            description: params.description.to_owned(),
            body: params.body.to_owned(),
        },
    );
    let tag_list = Tag::fetch_list_by_article_id(&conn, params.article_id);
    let profile = profile::service::fetch_profile_by_id(
        &conn,
        &FetchProfileById {
            me: params.me.to_owned(),
            id: article.author_id,
        },
    );
    let favorited_article_ids = favorite::service::fetch_favorited_article_ids_by_user_id(conn, params.me.id);
    let is_favorited = favorited_article_ids
        .to_owned()
        .into_iter()
        .any(|_id| _id == article.id);
    let favorite_info = FavoriteInfo {
        is_favorited,
        favorites_count: favorite::service::fetch_favorites_count_by_article_id(conn, article.id)
    };
    
    (article, profile, favorite_info, tag_list)
}
