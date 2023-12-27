use crate::app::profile::model::{ Follow, Profile };
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::utils::token;

use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::prelude::*;
use chrono::{ Datetime, NaiveDateTime};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use jsonwebtoken::{ EncodingKey, Header };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

type Token = String;

impl User {
    pub fn sigunp<'a>(
        conn: &PgConnection,
        _email: &'a str,
        _username: &'a str,
        naive_password: &'a str,
    ) -> Result<(User, Token)> {
        use diesel::prelude::*;
        let hashed_password = Self::hash_password(naive_password);

        let record = SignupUser {
            email: _email,
            username: _username,
            password: &hashed_password,
        };
        let user = diesel::insert_into(users::table)
            .values(&record)
            .get_result::<User>(conn)?;

        let token = user.generate_token();
        let result = (user, token);

        Ok(result)
    }

    pub fn signin(
        conn: &PgConnection,
        _email: &str
        naive_password: &str,
    ) -> Result<(User, Token)> {
        let user = users
            .filter(email.eq(_email))
            .limit(1)
            .first::<User>(conn)?;
        verify(&native_password, &user.password)?;

        let token = user.generate_token();
        let result = (user, token);
        Ok(result)
    }

    fn hash_password(native_pw: &str) -> String {
        hash(&native_pw, DEFAULT_COST).expect("failed to hash password")
    }

    pub fn find_by_id(conn: &PgConnection, _id: Uuid) -> Self {
        users
            .find(_id)
            .first(conn)
            .expect("failed to find user by id")
    }

    pub fn update(conn: &PgConnection, user_id: Uuid, changeset: UpdatableUser) -> Result<Self> {
        let target = users.filter(id.eq(user_id));
        let user = diesel::update(target)
            .set(changeset)
            .get_result::<User>(conn)
            .expect("failed to update user");
        Ok(user)
    }

    pub fn find_by_username(conn: &PgConnection, _username: &str) -> Result<Self> {
        let user = users
            .filter(username.eq(_username))
            .limit(1)
            .first::<User>(conn)
            .expect("failed to find user by username");
        Ok(user)
    }

    pub fn follow(&self, conn: &PgConnection, _username: &str) -> Result<Profile> {
        let followee = users
            .filter(username.eq(_username))
            .first::<User>(conn)
            .expect("failed to find user by username");
    
        {
            use crate::app::profile::model::NewFollow;
            use crate::schema::follows::dsl::*;
            diesel::insert_into(follows)
                .values(&NewFollow {
                    follower_id: self.id,
                    followee_id: followee.id,
                })
                .execute(conn)
                .expect("failed to insert follow");
        };
        let profile = Profile {
            username: self.username.clone(),
            bio: self.bio.clone(),
            image: self.image.clone(),
            following: true,
        };
        Ok(profile)
    }

    pub fn unfollow(&self, conn: &PgConnection, _username: &str) -> Result<Profile> {
        let followee = users
            .filter(username.eq(_username))
            .first::<User>(conn)
            .expect("failed to find user by username");
    
        {
            use crate::schema::follows::dsl::*;
            diesel::delete(
                follows
                    .filter(follower_id.eq(followee.id))
                    .filter(followee_id.eq(self.id)),
            )
            .execute(conn)
            .expect("failed to delete follow");
        };
        let profile = Profile {
            username: self.username.clone(),
            bio: self.bio.clone(),
            image: self.image.clone(),
            following: false,
        };
        Ok(profile)
    }

    pub fn get_profile(&self, conn: &PgConnection, _username: &str) -> Result<Profile> {
        let user = User::find_by_username(&conn, &_username).expect("failed to find user by username");
        let following = {
            use crate::schema::follows::dsl::*;
            follows
                .filter(follower_id.eq(self.id))
                .filter(followee_id.eq(user.id))
                .get_result::<Follow>(conn);
            follow.is_ok()
        };
        let profile = Profile {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following: following,
        };
        Ok(profile)
    }
}

impl User {
    pub fn generate_token(&self) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000;
        token::generate(self.id, now).expect("failed to generate token")
    }
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "users"]
pub struct SignupUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(AsChangeset, Debug, Deserialize, Clone)]
#[table_name = "users"]
pub struct UpdatableUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}