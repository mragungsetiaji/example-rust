use crate::app::article::model::{Article, NewArticle};
use crate::app::article::tag::model::{NewTag, Tag};
use diesel::pg::PgConnection;

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

fn create_tag_list(conn: &PgConnection, tag_list: &Option<Vec<String>>) -> Vec<Tag> {
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