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
    user (id) {
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

table! {
    user_profile (id) {
        id -> Uuid,
        avatar -> Nullable<Varchar>,
        nickname -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
    }
}

joinable!(article_news_category_rel -> article (article_id));
joinable!(article_news_category_rel -> category (cat_id));
joinable!(user -> user_profile (profile_id));

allow_tables_to_appear_in_same_query!(
    article,
    article_news_category_rel,
    category,
    user,
    user_profile,
);
