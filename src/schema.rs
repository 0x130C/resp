table! {
    article (id) {
        id -> Uuid,
        slug -> Varchar,
        title -> Varchar,
        raw_content -> Varchar,
        content -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        state -> Int2,
    }
}

table! {
    article_news_category_rel (article_id, cat_id) {
        article_id -> Uuid,
        cat_id -> Uuid,
    }
}

table! {
    category (id) {
        id -> Uuid,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    user_profile (id) {
        id -> Uuid,
        avatar -> Nullable<Varchar>,
        nickname -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        salt -> Varchar,
        actived_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        role -> Int2,
        state -> Int2,
        profile_id -> Nullable<Uuid>,
    }
}

joinable!(article_news_category_rel -> article (article_id));
joinable!(article_news_category_rel -> category (cat_id));
joinable!(users -> user_profile (profile_id));

allow_tables_to_appear_in_same_query!(
    article,
    article_news_category_rel,
    category,
    user_profile,
    users,
);
