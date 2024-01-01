use crate::app::article::model::Article;
use crate::app::user::model::User;
use crate::schema::comments;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Queryable, Associations, Debug, Clone)]
#[belongs_to(User, foreign_key = "author_id")]
#[belongs_to(Article, foreign_key = "article_id")]
#[table_name = "comments"]
pub struct Comment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Clone)]
#[table_name = "comments"]
pub struct CreateComment {
    pub body: String,
    pub author_id: Uuid,
    pub article_id: Uuid,
}

impl Comment {
    pub fn create(conn: &PgConnection, params: &CreateComment) -> Self {
        use diesel::prelude::*;
        diesel::insert_into(comments::table)
            .values(params)
            .get_result::<Comment>(conn)
            .expect("Error saving new comment")
    }

    pub fn delete(conn: &PgConnection, comment_id: &Uuid)  {
        use crate::schema::comments::dsl::*;
        use diesel::prelude::*;
        diesel::delete(comments.filter(id.eq(comment_id)))
            .execute(conn)
            .expect("Error deleting comment");
    }
}