extern crate actix_web;
extern crate listenfd;
extern crate chrono;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

mod models;
mod schema;

use listenfd::ListenFd;
use actix_web::{server, App, HttpRequest, HttpResponse, error, Error, Responder, http::Method, fs::{self, NamedFile}, Result};

fn login(req: HttpRequest<AppState>) -> impl Responder {
    match *req.method() {
        Method::GET => HttpResponse::Ok(),
        Method::POST => HttpResponse::Ok(),
        _ => HttpResponse::MethodNotAllowed()
    };
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

fn index(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let state: &AppState = req.state();
    let html = state
        .template
        .render("index.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template render error"))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}


fn parser(_: HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open("static/parser.html")?)
}

struct AppState {
    template: tera::Tera, // <- store tera template in application state
}

fn get_tera_template() -> tera::Tera {
    let mut tera:tera::Tera = compile_templates!("templates/**/*");
    //        tera.register_global_function("now", tera::builtins::global_functions::make_now_fn())
    tera
}

fn main() {
    let mut listenfd = ListenFd::from_env();

    let mut server = server::new(|| {

        let mut apps = vec![
            App::with_state(AppState{ template:get_tera_template() })
            .resource("/", |r| r.f(index))
            .resource("/parser", |r| r.f(parser))
            .resource("/login", |r| r.f(login))
            .handler("/", fs::StaticFiles::new("./static"))
            .boxed()
        ];
        if let Some(_) = option_env!("DEV_MODE") {
            apps.insert(0,
                App::new()
                    .prefix("/client")
                    .handler("/", fs::StaticFiles::new("./client").show_files_listing())
                    .boxed()
            );
        }
        apps
    });



    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8080").unwrap()
    };
    server.run();
}
