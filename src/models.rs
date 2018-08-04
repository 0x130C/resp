use super::schema::article;
use chrono::NaiveDateTime;

#[derive(DbEnum, Debug, PartialEq)]
pub enum StateEnum {
    Draft,
    Published
}

pub struct Article {
    id: String,
    slug: String,
    title: String,
    raw_content: String,
    content: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    state: StateEnum
}