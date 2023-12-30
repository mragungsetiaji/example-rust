use crate::app::user::model::User;
use crate::schema::follows;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// This line is using the derive attribute to automatically implement 
// the Queryable, Serialize, Deserialize, Debug, Clone, and Associations 
// traits for the Follow struct. This means that instances of Follow 
// can be queried from a database, serialized/deserialized 
// (converted to/from a data format like JSON), printed for debugging, cloned, 
// and used in associations with other structs.
#[derive(Queryable, Serialize, Deserialize, Debug, Clone, Associations)]
// This attribute is used by the Diesel ORM (Object-Relational Mapping) library. 
// It specifies that the Follow struct is associated with the User struct via 
// the followee_id and follower_id foreign keys. This sets up a relationship 
// where a Follow belongs to a User.
#[belongs_to(User, foreign_key = "followee_id", foreign_key = "follower_id")]
// This attribute is also used by Diesel. It specifies that the Follow struct 
// should be associated with the follows table in the database. 
// This means that when you query the follows table, Diesel will return 
// results as instances of Follow.
#[table_name = "follows"]
pub struct Follow {
    pub follower_id: Uuid,
    pub followed_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "follows"]
pub struct NewFollow {
    pub follower_id: Uuid,
    pub followee_id: Uuid,
}