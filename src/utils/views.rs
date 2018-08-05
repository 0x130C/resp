use actix_web::{HttpRequest, HttpResponse, Error, http::Method, error};
use AppState;
use tera;

pub fn render_template(req: &HttpRequest<AppState>, template_name: &str) -> Result<HttpResponse, Error>{
    let state: &AppState = req.state();
    let html = state
        .template
        .render(template_name, &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template render error"))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}