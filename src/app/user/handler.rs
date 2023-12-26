use crate::schema::users;
use super::model::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupReqUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupRes {
    pub user: SignupResUser,
}

impl SignupRes {
    pub fn from(user: User) -> Self {
        Self {
            user: SignupResUser {
                email: user.email,
                token: user.token,
                username: user.username,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SignupResUser {
    pub email: String,
    pub token: String,
    pub username: String,
}

