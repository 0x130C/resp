#![allow(dead_code)]
#![allow(proc_macro_derive_resolution_fallback)]
extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate jsonwebtoken;
extern crate listenfd;
extern crate rustls;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate uuid;
extern crate validator;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_derive_enum;
#[macro_use] extern crate failure;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate tera;
#[macro_use] extern crate validator_derive;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate jsonwebtoken as jwt;

mod db;
mod schema;
mod utils;
mod models;
mod views;
mod controllers;
mod token;
mod auth;
mod routes;
//mod middlewares;
//mod extractors;

use listenfd::ListenFd;

use actix::SystemRunner;
use actix_web::actix::*;
use actix_web::{server, middleware, App};
use actix_web::{error, Error, Responder, Result};
use actix_web::{http, HttpRequest, HttpResponse};
use actix_web::fs::{self, NamedFile};
use actix_web::pred;
use utils::http::redirect;
use utils::views::render_template;

// SSL
use rustls::{ServerConfig, NoClientAuth};
use rustls::internal::pemfile::{certs, rsa_private_keys};
use std::io::BufReader;
use std::fs::File;

use views::login;

use db::{create_pool, DbExecutor};

pub struct AppState {
    template: tera::Tera, // <- store tera template in application state
    db: Addr<DbExecutor>
}



fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    render_template(&req, "index.html")
}


fn get_tera_template() -> tera::Tera {
    let tera:tera::Tera = compile_templates!("templates/**/*");
    //TODO: Create function addMinInProduct to add .min in assets file, ex: app.css -> app.min.css
    //        tera.register_global_function("now", tera::builtins::global_functions::make_now_fn())
    tera
}
pub struct Config {

}

struct ServerInner {
    runner: SystemRunner
}

pub struct Server {
    config: Config,
    inner: ServerInner
}

impl Server {
    pub fn new(config: Config) -> Self {
        let runner = System::new("resp-server");
        let mut listenfd = ListenFd::from_env();

        let pool = create_pool();
        let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));
        let mut server = server::new(move || {

            let mut apps = vec![
                App::with_state(AppState{ template:get_tera_template(), db: addr.clone() })
                    .middleware(middleware::Logger::default())
                    .resource("/", |r| {
                        r.name("homepage");
                        r.f(index)
                    })
                    .resource("/login", |r| {
                        r.method(http::Method::GET).f(login::get);
                        r.method(http::Method::POST).f(login::post);
                    })
                    .handler("/", fs::StaticFiles::new("./static").unwrap())
                    .scope("/blog", routes::blog::blog_routes)
                    .boxed()
            ];
            if let Some(_) = option_env!("DEV_MODE") {
                apps.insert(0,
                            App::new()
                                .prefix("/client")
                                .handler("/", fs::StaticFiles::new("./client").unwrap().show_files_listing())
                                .boxed()
                );
            }
            apps
        });

        match option_env!("DEV_MODE") {
            Some(_) => {

                server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
                    server.listen(l)
                } else {
                    server.bind("127.0.0.1:8080").unwrap()
                };

            },
            None => {
                // load ssl keys
                let mut svr_config = ServerConfig::new(NoClientAuth::new());
                let cert_file = &mut BufReader::new(File::open("tls/cert.pem").unwrap());
                let key_file = &mut BufReader::new(File::open("tls/key.pem").unwrap());
                let cert_chain = certs(cert_file).unwrap();
                let mut keys = rsa_private_keys(key_file).unwrap();
                svr_config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
                server = server.bind_rustls("127.0.0.1:9443",svr_config).unwrap()
            }

        }

        server.start();
        Server {
            config,
            inner: ServerInner {
                runner
            }
        }
    }

    pub fn start(self) -> i32 {
        info!("starting up");
        self.inner.runner.run()

    }
}
