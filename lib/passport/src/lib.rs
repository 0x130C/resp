#[macro_use] extern crate log;
extern crate actix_web;
extern crate futures;
extern crate parking_lot;
#[macro_use]
extern crate failure;
extern crate base64;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;
use parking_lot::Mutex;
use std::cell::RefCell;
use std::ops::Deref;
use std::result;
use actix_web::{FromRequest, HttpRequest, HttpResponse, Error, Result, middleware::{Middleware, Response,Started, Finished}};
use actix_web::http::header::{self, HeaderValue};
use futures::future::{err as FutErr, ok as FutOk, FutureResult};
use futures::Future;


pub mod strategies;
pub mod error;
pub mod config;

use config::PassportConfig;
use strategies::BasicStrategy;
use strategies::PassportStrategy;
use strategies::StrategyInfo;

use PassportStrategies::*;



pub struct Passport<S> {
    config: PassportConfig,
    strategies: PassportStrategies<S>,
    handler: Box<AuthHandler<S>>
}

pub enum PassportStrategies<S> {
    NotAvailable,
    Available(Arc<PassportCell<S>>)
}

impl<S> Passport<S> {

    pub fn new<F>(h: F) -> Self
        where F: Fn(StrategyInfo, &HttpRequest<S>) -> Result<Response>  + 'static{
        Passport {
            config: PassportConfig::default(),
            strategies: PassportStrategies::Available(Arc::new(PassportCell(Mutex::new(HashMap::new())))),
            handler: Box::new(h)
        }
    }

    pub fn with_strategies_manager(mut self) -> Self {
        self.config.manager_strategies = true;
        self
    }

    pub fn register(mut self, strategy_name: &str, strategy: Box<PassportStrategy<S>>) -> Self {
        self.strategies.add(strategy_name, strategy);
        self
    }

    pub fn unregister(mut self, strategy_name: &str) -> Self {
        self.strategies.remove(strategy_name);
        self
    }
}

impl<S: 'static> Middleware<S> for Passport<S> {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        if let Available(ref arc) = self.strategies {
            if self.config.manager_strategies {
                req.extensions_mut().insert(arc.clone());
            }
        }
        Ok(Started::Done)
    }

    fn response(
        &self, req: &HttpRequest<S>, resp: HttpResponse,
    ) -> Result<Response> {
        if let Some(t) = req.extensions().get::<Arc<Payload>>() {
            println!("Middleware main: Get gate type successed.");
            let payload = t;
            let strategy_name: &str = payload._type.as_ref();
            match strategy_name {
                "Basic" => {
                    println!("Middleware main: Basic gate type.");
                },
                _ => {}
            }
            if let Available(ref arc) = self.strategies {
                let guard = arc.0.lock();
                let strategy = guard.get(strategy_name).expect(format!("Strategy {} not registered", strategy_name).as_ref());
                let fut = strategy.extract_info(req);
                fut.from_err()
                .and_then(|info|{
                    (self.handler)(info, req)
                });
           }
        }
        println!("Middleware main: response");
        Ok(Response::Done(resp))
    }

    fn finish(&self, req: &HttpRequest<S>, resp: &HttpResponse) -> Finished {
        if self.config.manager_strategies {
            req.extensions_mut().remove::<Arc<PassportCell<S>>>();
        }
        Finished::Done
    }
}
pub type AuthHandler<S> = Fn(StrategyInfo, &HttpRequest<S>) -> Result<Response>;

pub struct PassportCell<S>(Mutex<HashMap<String, Box<PassportStrategy<S>>>>);


impl<S> PassportStrategies<S> {
    pub fn add(&self, strategy_name: &str, strategy: Box<PassportStrategy<S>>) {
        if let Available(ref arc) = self {
            let  mut strategies = arc.0.lock();
            match strategies.get(strategy_name) {
                Some(_) => {
                    warn!("Strategy {} really registered!", strategy_name);
                },
                _ => {
                    strategies.insert(strategy_name.to_string(), strategy);
                }
            }
        }
    }

    pub fn remove(&self, strategy_name: &str)  {
        if let Available(ref arc) = self {
            if let None = arc.0.lock().remove(strategy_name) {
                warn!("Strategy {} not register before!", strategy_name);
            }
        }
    }

    pub fn has_strategy(&self, strategy_name: &str) -> bool {
        if let Available(ref arc) = self {
            return arc.0.lock().contains_key(strategy_name);
        }
        false
    }
}


pub trait RequestPassportStrategies<S> {
    fn passport_strategies(&self) -> PassportStrategies<S>;
}

impl<S: 'static> RequestPassportStrategies<S> for HttpRequest<S> {
    fn passport_strategies(&self) -> PassportStrategies<S> {
        match self.extensions().get::<Arc<PassportCell<S>>>() {
            Some(t) => Available(Arc::clone(&t)),
            None => NotAvailable
        }
    }
}

impl<S: 'static> FromRequest<S> for PassportStrategies<S> {
    type Config = ();
    type Result = Self;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _cfg: &Self::Config) -> Self::Result {
        req.passport_strategies()
    }
}

pub struct Config {
    success_redirect: Option<String>,
    failure_redirect: Option<String>,
    success_flash: Option<String>,
    failure_flash: Option<String>,
    store_session: bool
}

impl Default for Config {
    fn default() -> Self {
        Config {
            success_redirect: None,
            failure_redirect: None,
            success_flash: None,
            failure_flash: None,
            store_session: true
        }
    }
}

pub struct Payload {
    _type: String,
    config: Config
}

pub struct Authenticate(Arc<Payload>);

impl Authenticate {
    pub fn new(_type: &str, config: Config) -> Self {
        let _type = _type.to_owned();
        Authenticate(Arc::new(Payload {
            _type,
            config
        }))
    }
}

macro_rules! auth {
        ($_type:expr) => { Authenticate::new($_type, Config::default())
        };
        ($_type:expr, {}) => (
            auth!($_type)
        );
        ($_type:expr, {$($arg:ident: $val:expr),+,}) => (
            auth!($_type, {$($arg:$val),+})
        );
        ($_type:expr, {$($arg:ident: $val:expr),+}) => (
            Authenticate::new($_type, Config{
                $($arg : $val),+
                , .. Config::default()
            })
        )
    }



impl<S: 'static> Middleware<S> for Authenticate {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        req.extensions_mut().insert(self.0.clone());;
        println!("Middleware Authenticate");
        Ok(Started::Done)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header::{self, Header};
    use actix_web::{App, test};
    use actix_web::client::ClientResponse;
    use actix_web::HttpMessage;

    #[test]
    fn default_passport_should_run_without_error() {

        let mut srv = test::TestServer::with_factory(|| {
            App::new()
                .middleware(Passport::new(|_, _|{
                    Ok(Started::Done)
                }))
                .resource("/", |r| {
                    r.f(|req| {
                        "test"
                    })
                })
        });

        let mut request = srv.get().uri(srv.url("/")).finish().unwrap();
        request.headers_mut().append(header::AUTHORIZATION, header::HeaderValue::from_static("Basic abc"));
        let response  = srv.execute(request.send()).unwrap();
        assert!(response.status().is_success());

    }

    #[test]
    fn passport_with_basic_stragery() {
        let mut srv = test::TestServer::with_factory(|| {
            App::new()
                .middleware(Passport::new(|_,_|{
                    Ok(Started::Done)
                })
                    .register(
                        "Basic",
                        Box::new(BasicStrategy{})
                    )
                )
                .resource("/", |r| {
                    r.middleware(auth!("Basic"));
                    r.f(|req| {
                        "test"
                    })
                })
        });

        let mut request = srv.get().uri(srv.url("/")).finish().unwrap();
        request.headers_mut().append(header::AUTHORIZATION, header::HeaderValue::from_static("Basic abc:"));
        let response  = srv.execute(request.send()).unwrap();
        assert!(response.status().is_success());

    }

    #[test]
    fn passport_extractor() {
        let mut srv = test::TestServer::with_factory(|| {
            App::new()
                .middleware(Passport::new(|_, _| {
                    Ok(Started::Done)
                }))
                .resource("/", |r| {
                    r.with(|strategies: PassportStrategies<()>| {
                        assert!(true);
                        "test"
                    })
                })
        });

        let request = srv.get().uri(srv.url("/")).finish().unwrap();
        let response = srv.execute(request.send()).unwrap();
        assert!(response.status().is_success());
    }

    #[test]
    fn passport_extractor_dynamic_register() {
        let mut srv = test::TestServer::with_factory(|| {
            App::new()
                .middleware(Passport::new(|_,_|{
                    Ok(Started::Done)
                }).with_strategies_manager())
                .resource("/", |r| {
                    r.with(|mut strategies: PassportStrategies<()>| {
                        if let Available(_) = strategies {
                            strategies.add(
                                "Basic",
                                Box::new(BasicStrategy{})
                            );
                            assert!(strategies.has_strategy("Basic"));
                        }
                        "test"
                    })
                })
        });

        let request = srv.get().uri(srv.url("/")).finish().unwrap();
        let response = srv.execute(request.send()).unwrap();
        assert!(response.status().is_success());
    }

}