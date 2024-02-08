use crate::app::user::model::User;
use crate::error::AppError;
use crate::schema::articles::dsl::*;
use crate::schema::articles;
use crate::utils::converter;

use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{
    Deserialize, Serialize
};
use uuid::Uuid;

// Identifiable
// This trait is used to mark a struct as representing a single row 
// in a database table. This trait requires that your struct has 
// a field named id or a tuple of fields which together form 
// a composite primary key. The id field is typically of 
// type i32 or Uuid and is often auto-incremented by the database. 
// By deriving Identifiable, you can use methods provided 
// by Diesel that require the struct to be identifiable by 
// a unique key.
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Associations, Clone)]
#[belongs_to(User, foreign_key = "author_id")]
#[table_name = "articles"]
pub struct Article {
    pub id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Article {
    pub fn create(conn: &PgConnection, record: &NewArticle) -> Result<Self, AppError> {
        let article = diesel::insert_into(articles::table)
            .values(record)
            .get_result::<Article>(conn)?;
        Ok(article)
    }

    pub fn update(conn: &PgConnection, article_id: &Uuid, record: &UpdateArticle) -> Result<Self, AppError> {
        let article = diesel::update(articles.filter(id.eq(article_id)))
            .set(record)
            .get_result::<Article>(conn)?;
        Ok(article)
    }

    pub fn convert_title_to_slug(_title: &str) -> String {
        converter::to_kebab(_title)
    }
}

#[derive(Insertable, Clone)]
#[table_name = "articles"]
pub struct NewArticle {
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
}

#[derive(AsChangeset)]
#[table_name = "articles"]
pub struct UpdateArticle {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}