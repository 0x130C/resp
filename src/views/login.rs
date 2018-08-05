use actix_web::{HttpRequest, HttpResponse, Error, http::Method, error};
use utils::views::render_template;
use AppState;


pub fn get(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error>  {
    render_template(&req, "login.html")
}
pub fn post(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error>  { // impl Responder
    Ok(HttpResponse::TemporaryRedirect().header("Location", "/").finish())
}