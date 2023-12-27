table! {
    articles (id) {
        id -> Uuid,
        author_id -> Uuid,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    follows (follower_id, followed_id) {
        follower_id -> Uuid,
        followed_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tags (id) {
        id -> Uuid,
        article_id -> Uuid,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        username -> Text,
        password -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(articles -> users (author_id));
joinable!(tags -> articles (article_id));

allow_tables_to_appear_in_same_query!(
    articles,
    follows,
    tags,
    users,
);