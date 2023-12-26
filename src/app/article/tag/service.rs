use diesel::pg::PgConnection;
use diesel::prelude::*;

use super::model::{NewTag, Tag};
use crate::schema;

pub fn create_tag<'a>(conn: &mut PgConnection, name: &'a str) -> Tag {
    use schema::tags;
    let new_tag = NewTag { name: name };
    diesel::insert_into(tags::table)
        .values(&new_tag)
        .get_result(conn)
        .expect("Error saving new tag")
}