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
        account -> Varchar,
        password -> Varchar,
        salt -> Varchar,
        nickname -> Varchar,
        avatar -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
        signup_time -> Timestamp,
        role -> Int2,
        state -> Int2,
    }
}

joinable!(article_news_category_rel -> article (article_id));
joinable!(article_news_category_rel -> category (cat_id));

allow_tables_to_appear_in_same_query!(
    article,
    article_news_category_rel,
    category,
    user,
);
