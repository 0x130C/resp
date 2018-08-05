use actix_web::{HttpResponse};

#[inline]
pub fn redirect(location: &str) -> HttpResponse {
    HttpResponse::TemporaryRedirect().header("Location", location).finish()
}