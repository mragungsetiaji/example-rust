use crate::app::article::model::Article;
use crate::app::user::model::User;
use crate::schema::favorites;
use chrono::NaiveDateTime;
use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations, Clone, Debug)]
#[belongs_to(Article, foreign_key = "article_id")]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "favorites"]
pub struct Favorite {
    pub id: Uuid,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Favorite {

    // usize is the number of rows affected by the query
    pub fn favorite(conn: &PgConnection, record: &FavoriteAction) -> usize {
        let item = diesel::insert_into(favorites::table)
            .values(record)
            .execute(conn)
            .expect("could not do favorite.");

        item
    }
    
    pub fn unfavorite(conn: &PgConnection, record: &UnfavoriteAction) -> usize {
        let item = diesel::delete(favorites::table)
            .filter(favorites::user_id.eq_all(record.user_id))
            .filter(favorites::article_id.eq_all(record.article_id))
            .execute(conn)
            .expect("could not do unfavorite.");

        item
    }
}

#[derive(Insertable)]
#[table_name = "favorites"]
pub struct FavoriteAction {
    pub user_id: Uuid,
    pub article_id: Uuid,
}

pub struct UnfavoriteAction {
    pub user_id: Uuid,
    pub article_id: Uuid,
}