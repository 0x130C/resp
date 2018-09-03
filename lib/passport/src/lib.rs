#[macro_use] extern crate log;
extern crate actix_web;
extern crate futures;
extern crate parking_lot;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;
use parking_lot::Mutex;
use std::cell::RefCell;
use std::ops::Deref;
use std::result;
use actix_web::{FromRequest, HttpRequest, HttpResponse, Error, Result, middleware::{Middleware, Response,Started}};
use actix_web::http::header::{self, HeaderValue};
use futures::future::{err as FutErr, ok as FutOk, FutureResult};


pub mod strategies;
pub mod error;
pub mod config;

use config::PassportConfig;
use strategies::BasicStrategy;
use strategies::PassportStrategy;
use strategies::StrategyInfo;





pub struct Passport<S> {
    config: PassportConfig,
    strategies: PassportStrategies<S>,
    handler: Box<AuthHandler<S>>
}

impl<S> Passport<S> {

    pub fn new<F>(h: F) -> Self
        where F: Fn(&HttpRequest<S>) -> Result<Started>  + 'static{
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
        match self.strategies {
            PassportStrategies::Available(ref arc) => {
                if self.config.manager_strategies {
                    req.extensions_mut().insert(arc.clone());
                }

                Ok(Started::Done)
            }
            PassportStrategies::NotAvailable => {
                Ok(Started::Done)
            }
        }

    }

    fn response(
        &self, req: &HttpRequest<S>, resp: HttpResponse,
    ) -> Result<Response> {
        if self.config.manager_strategies {
            req.extensions_mut().remove::<Arc<PassportCell<S>>>();
        }
        Ok(Response::Done(resp))
    }
}

pub type AuthHandler<S> = Fn(&HttpRequest<S>) -> Result<Started>;

pub struct PassportCell<S>(Mutex<HashMap<String, Box<PassportStrategy<S>>>>);

pub enum PassportStrategies<S> {
    NotAvailable,
    Available(Arc<PassportCell<S>>)
}

impl<S> PassportStrategies<S> {
    pub fn add(&self, strategy_name: &str, strategy: Box<PassportStrategy<S>>) {
        if let PassportStrategies::Available(ref arc) = self {
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
        if let PassportStrategies::Available(ref arc) = self {
            if let None = arc.0.lock().remove(strategy_name) {
                warn!("Strategy {} not register before!", strategy_name);
            }
        }
    }
}


pub trait RequestPassportStrategies<S> {
    fn passport_strategies(&self) -> PassportStrategies<S>;
}

impl<S: 'static> RequestPassportStrategies<S> for HttpRequest<S> {
    fn passport_strategies(&self) -> PassportStrategies<S> {
        if let Some(t) = self.extensions().get::<Arc<PassportCell<S>>>() {
            return PassportStrategies::Available(Arc::clone(&t));
        }
        PassportStrategies::NotAvailable
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
                .middleware(Passport::new(|_|{
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
                .middleware(Passport::new(|_|{
                    Ok(Started::Done)
                })
                    .register(
                        "Basic",
                        Box::new(BasicStrategy{})
                    )
                )
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
    fn passport_extractor() {
        let mut srv = test::TestServer::with_factory(|| {
            App::new()
                .middleware(Passport::new(|_| {
                    Ok(Started::Done)
                }))
                .resource("/", |r| {
                    r.with(|strategies: PassportStrategies<()>| {
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
                .middleware(Passport::new(|_|{
                    Ok(Started::Done)
                }))
                .resource("/", |r| {
                    r.with(|mut strategies: PassportStrategies<()>| {
                        strategies.add(
                            "Basic",
                            Box::new(BasicStrategy{})
                        );
                        "test"
                    })
                })
        });

        let request = srv.get().uri(srv.url("/")).finish().unwrap();
        let response = srv.execute(request.send()).unwrap();

        let request = srv.get().uri(srv.url("/")).finish().unwrap();
        let response = srv.execute(request.send()).unwrap();
        assert!(response.status().is_success());
    }

}