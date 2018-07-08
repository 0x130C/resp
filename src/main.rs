extern crate actix_web;
extern crate listenfd;


use listenfd::ListenFd;
use actix_web::{server, App, HttpRequest, HttpResponse, Responder, http::Method, fs::{self, NamedFile}, Result};

fn hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

fn index(_: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}


fn parser(_: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("static/parser.html")?)
}

fn main() {
    let mut listenfd = ListenFd::from_env();

    let mut server = server::new(|| {
        let mut apps = vec![
            App::new()
            .resource("/", |r| r.f(index))
            .resource("/parser", |r| r.f(parser))
            .resource("/hello/{name}", |r| r.method(Method::GET).f(hello))
            .handler("/", fs::StaticFiles::new("./static"))
        ];
        if let Some(_) = option_env!("DEV_MODE") {
            apps.insert(0,
                App::new()
                    .prefix("/dev")
                    .handler("/", fs::StaticFiles::new("./client/assets").show_files_listing())
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
