use super::model::Profile;
use crate::app::user::model::User;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub struct FetchProfileByName {
    pub me: User,
    pub username: String,
}
pub fn fetch_by_name(conn: &PgConnection, params: &FetchProfileByName) -> Profile {
    let FetchProfileByName { me, username } = params;
    let followee = User::find_by_username(&conn, username).expect("couldn't find user.");
    let profile = fetch_profile_by_id(
        &conn,
        &FetchProfileById {
            me: me.to_owned(),
            id: followee.id,
        },
    );
    profile
}

pub struct FetchProfileById {
    pub me: User,
    pub id: Uuid,
}

pub fn fetch_profile_by_id(conn: &PgConnection, params: &FetchProfileById) -> Profile {
    let FetchProfileById { me, id } = params;
    let is_following = me.is_following(&conn, id);
    let profile = Profile {
        username: me.username.to_owned(),
        bio: me.bio.to_owned(),
        image: me.image.to_owned(),
        following: is_following,
    };
    profile
}