use crate::app::article::model::Article;
use crate::schema::tags;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::Insertable;
use diesel::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// derive is an attribute that generates code for certain traits. 
// It allows you to automatically implement commonly used traits 
// for your custom data types.
//
// Queryable: This trait comes from the Diesel crate and 
// indicates that a struct can be used to load a row from the 
// database.
// Debug: This trait allows instances of the struct to be 
// printed for debugging purposes using the {:?} or {:#?} 
// format specifier.
// Serialize: This trait comes from the Serde crate and allows 
// the struct to be serialized into a format such as JSON.
// Deserialize: This trait also comes from the Serde crate 
// and allows the struct to be deserialized from a format 
// such as JSON.
#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Clone, Associations)]
#[belongs_to(Article, foreign_key = "article_id")]
#[table_name = "tags"]
pub struct Tag {
    pub id: Uuid,
    pub article_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Tag {

    pub fn fetch_list_by_article_id(conn: &PgConnection, _article_id: Uuid) -> Vec<Self> {
        use crate::schema::tags;
        use crate::schema::tags::dsl::*;
        use diesel::prelude::*;
        let list = tags
            .filter(tags::article_id.eq(_article_id))
            .get_results::<Self>(conn)
            .expect("Error loading tags");
        list
    }

    pub fn list(conn: &PgConnection) -> anyhow::Result<Vec<Self>> {
        use crate::schema;
        use diesel::prelude::*;
        use schema::tags::dsl::*;

        match tags.load::<Self>(conn) {
            Ok(list) => Ok(list),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }

    pub fn create_list(conn: &PgConnection, records: Vec<NewTag>) -> Vec<Self> {
        use crate::schema::tags::dsl::*;
        let tags_list = diesel::insert_into(tags)
            .values(records)
            .get_results::<Tag>(conn)
            .expect("Error saving new tag");
        tags_list
    }
}

//  &'a str is a string slice, which is a reference to a string. 
// It's used here because it's more flexible and can refer to any 
// string data stored elsewhere in memory, whether it's a String,
// a string literal, or a part of another string. 
// The 'a is a lifetime specifier, indicating that the reference 
// to the string data has a certain lifetime 'a.
// Using &str can be more efficient than using String because 
// it doesn't involve memory allocation. However, it means that 
// NewTag doesn't own the string data it refers to. The data must 
// be kept alive elsewhere in your program for as long as the 
// NewTag instance needs it.
#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag<'a>{
    pub name: &'a str,
    pub article_id: &'a Uuid,
}