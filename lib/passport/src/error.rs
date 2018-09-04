use actix_web::error::{ResponseError};
use actix_web::HttpResponse;
use actix_web::http::{StatusCode, header};
use failure::Fail;
use base64;
use std::str;

#[derive(Fail, Debug)]
pub enum ExtractError {
    #[fail(display = "Invalid header")]
    Invalid,
    #[fail(display = "Missing required field")]
    MissingField(&'static str),
    #[fail(display = "Missing required scheme")]
    MissingScheme,
    #[fail(display = "ToStr Error {}", _0)]
    ToStrError(header::ToStrError),
    #[fail(display = "Decode Error {}", _0)]
    Base64DecodeError(base64::DecodeError),
    #[fail(display = "Utf8 Error {}", _0)]
    Utf8Error(str::Utf8Error),
}
impl From<header::ToStrError> for ExtractError {
    fn from(e: header::ToStrError) -> Self {
        ExtractError::ToStrError(e)
    }
}
impl From<base64::DecodeError> for ExtractError {
    fn from(e: base64::DecodeError) -> Self {
        ExtractError::Base64DecodeError(e)
    }
}
impl From<str::Utf8Error> for ExtractError {
    fn from(e: str::Utf8Error) -> Self {
        ExtractError::Utf8Error(e)
    }
}

impl ResponseError for ExtractError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(StatusCode::BAD_REQUEST)
    }
}