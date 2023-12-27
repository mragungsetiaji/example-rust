use crate::app::user::model::User;
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserResponse {
    pub user: AuthUser,
}

impl UserResponse {
    pub fn from(user: User, token: String) -> Self {
        Self {
            user: AuthUser {
                email: user.email,
                token: token,
                username: user.username,
                bio: user.bio,
                image: user.image,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthUser {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}