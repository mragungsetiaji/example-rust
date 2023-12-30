use super::model::{ Comment, CreateComment };
use crate::app::profile::model::Profile;
use crate::app::profile::service::{ fetch_profile_by_id, FetchProfileById };    
use crate::app::user::model::User;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub struct CreateCommentService {
    pub body: String,
    pub article_id: Uuid,
    pub author: User,
}

pub fn create(conn: &PgConnection, params: &CreateCommentService) -> Comment {
    let CreateCommentService { body, article_id, author } = params;
    let comment = Comment::create(
        &conn,
        &CreateComment {
            body: body.to_string(),
            author_id: author.id,
            article_id: article_id.to_owned(),
        },
    );
    let profile = fetch_profile_by_id(
        &conn,
        &FetchProfileById {
            me: author.to_owned(),
            id: author.id,
        },
    );
    (comment, profile)
}

pub fn fetch_comments_list(conn: &PgConnection, user: &User) -> Vec<(Comment, Profile)> {
    use crate::schema::comments;
    use crate::schema::comments::dsl::*;
    use crate::schema::follows;
    use crate::schema::users;
    use diesel::prelude::*;
    let _comments = comments
        .inner_join(users::table)
        .filter(comments::article_id.eq(article_id))
        .get_result::<(Comment, User)>(conn)
        .expect("Error loading comments");
    
    let _comments = _comments
        .into_iter()
        .map(|_comment, user| {
            let profile = fetch_profile_by_id(
                &conn,
                &FetchProfileById {
                    me: user.to_owned(),
                    id: _user.id,
                },
            );
            (_comment.to_owned(), profile)
        })
        .collect::<Vec<(Comment, Profile)>>();

    _comments
}