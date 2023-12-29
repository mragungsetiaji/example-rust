use crate::app::comment::model::Comment;
use crate::app::profile::model::Profile;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SingleCommentResponse {
    pub comment: InnerComment,
}

impl SingleCommentResponse {
    pub fn from(comment: Comment, profile: Profile) -> Self {
        Self {
            comment: InnerComment {
                id: comment.id,
                body: comment.body,
                author: InnerAuthor {
                    username: profile.username,
                    bio: profile.bio,
                    image: profile.image,
                    following: profile.following,
                },
                created_at: comment.created_at,
                updated_at: comment.updated_at,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct InnerComment {
    pub id: Uuid,
    pub body: String,
    pub author: InnerAuthor,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct InnerAuthor {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}