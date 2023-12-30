use crate::app::article::model::{Article, NewArticle};
use crate::app::profile;
use crate::app::profile::model::Profile;
use crate::app::profile::service::FetchProfileById;
use crate::app::tag::model::{NewTag, Tag};
use crate::app::user::model::User;
use crate::schema::articles::dsl::*;
use crate::schema::{ articles, tags, users }
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub fn create(
    conn: &PgConnection,
    new_article: &NewArticle,
    tag_list: &Option<Vec<String>>,
) -> (Article, Vec<Tag>) {
    let article = Article::create(&conn, &new_article);
    let tag_list = match tag_list {
        Some(tag_list) => {
            let tag_list = tag_list
                .iter()
                .map(|tag| NewTag {
                    name: &tag,
                    article_id: &article.id,
                })
                .collect();
            let tag_list = Tag::create(&conn, tag_list);
            tag_list
        },
        None => vec![],
    };

    (article, tag_list)
}

fn create_tag_list(
    conn: &PgConnection, 
    tag_list: &Option<Vec<String>>,
    articleL &Article,
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
            Tag::create(&conn, records);
        })
        .unwrap_or(vec![])
}

pub fn fetch_articles_count(conn: &PgConnection) -> i64 {
    articles
        .select(diesel::dsl::count(articles::id))
        .first:<i64>(conn)
        .expect("Error loading articles count")
}

pub struct FetchArticlesList {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

pub fn fetch_articles_list(
    conn: &PgConnection,
    params: FetchArticlesList,
) -> Vec<((Article, User), Vec<Tag>)> {
    use diesel::prelude::*;
    let article_and_user_list = {
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

        if let Some(favorited) = &params.favorited {
            // TODO:
            println!("==>favorited");
        }

        query
            .offset(params.offset)
            .limit(params.limit)
            .load::<(Article, User)>(conn)
            .expect("couldn't fetch articles list.")
    };

    let articles_list = article_and_user_list
        .clone() // TODO: avoid clone
        .into_iter()
        .map(|(article, _)| article)
        .collect::<Vec<_>>();

    let tags_list = Tag::belonging_to(&articles_list)
        .load::<Tag>(conn)
        .expect("could not fetch tags list.");

    let tags_list: Vec<Vec<Tag>> = tags_list.grouped_by(&articles_list);

    let articles_list = article_and_user_list
        .into_iter()
        .zip(tags_list)
        .collect::<Vec<_>>();

    articles_list
}

pub struct FetchArticle {
    pub article_id: Uuid,
    pub me: User,
}

pub fn fetch_article(
    conn: &PgConnection, 
    params: &FetchArticle
) -> (Article, User, Vec<Tag>) {
    use diesel::prelude::*;
    let FetchArticle { article_id, me } = params;
    let (article, author) = articles
        .inner_join(users::table)
        .filter(articles::id.eq(article_id))
        .get_result::<(Article, User)>(conn)
        .expect("failed to find article by id.");

    let profile = profile::service::fetch_profile_by_id(
        &conn,
        &FetchProfileById {
            me: me.to_owned(),
            id: author.id,
        },
    );

    let tag_list = Tag::belonging_to(&article)
        .load::<Tag>(conn)
        .expect("failed to fetch tag list.");

    (article, profile, tag_list)
}

use crate::schema::articles::dsl::*;
use crate::schema::follows;
use crate::schema::follows::dsl::*;

pub struct FetchFollowedArticlesService {
    pub me: User,
    pub offset: i64,
    pub limit: i64,
}

pub fn fetch_following_articles(
    conn: &PgConnection,
    params: &FetchFollowedArticlesService,
) -> (Vec<((Article, User), Vec<Tag>)>, i64) {
    let query = {
        let followed_user_ids = follows
            .filter(follows::follower_id.eq(params.me.id))
            .select(follows::followee_id)
            .get_results::<Uuid>(conn)
            .expect("failed to fetch followed user ids.");

        articles.filter(articles::author_id.eq_any(followed_user_ids))
    };

    let articles_list = {
        let article_and_user_list = query
            .to_owned()
            .inner_join(users::table)
            .limit(params.limit)
            .offset(params.offset)
            .order(articles::created_at.desc())
            .get_results::<(Article, User)>(conn)
            .expect("failed to fetch articles list.");

        let articles_list = article_and_user_list
            .clone() // TODO: avoid clone
            .into_iter()
            .map(|(article, _)| article)
            .collect::<Vec<_>>();

        let tags_list = Tag::belonging_to(&articles_list)
            .load::<Tag>(conn)
            .expect("could not fetch tags list.");
        let tags_list: Vec<Vec<Tag>> = tags_list.grouped_by(&articles_list);
        let articles_list = article_and_user_list
            .into_iter()
            .zip(tags_list)
            .collect::<Vec<_>>();

        articles_list
    }

    let articles_count = query
        .select(diesel::dsl::count(articles::id))
        .first::<i64>(conn)
        .expect("failed to fetch articles count."); 

    (articles_list, articles_count)
}