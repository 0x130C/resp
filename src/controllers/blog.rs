use actix_web::{HttpRequest, HttpResponse, Error, http::Method, error};
use utils::views::render_template;
use AppState;


pub fn home(req: HttpRequest<AppState>) -> Result<HttpResponse, Error>  {
    render_template(&req, "blog/index.html")
}