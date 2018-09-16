#![allow(dead_code)] //TODO Remove when release
#![allow(unused_imports)]
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
use std::ops::{Deref, DerefMut};
use std::result;
use actix_web::{FromRequest, HttpRequest, HttpResponse, Error, Result, middleware::{Middleware, Response,Started, Finished}};
use actix_web::http::header::{self, HeaderValue};
use futures::future::{err as FutErr, ok as FutOk, FutureResult};
use futures::Future;
use actix_web::AsyncResponder;
use actix_web::error::ErrorUnauthorized;
use std::marker::PhantomData;

pub mod strategies;
pub mod error;
pub mod config;

use config::PassportConfig;
use strategies::BasicStrategy;
use strategies::PassportStrategy;
use strategies::StrategyInfo;

use PassportStrategies::*;



pub struct Passport<S> {
    inner: PassportInner<S>,
    finished: bool,
}
struct PassportInner<S> {
    config: PassportConfig,
    strategies: PassportStrategies<S>,
    handler: Arc<Box<AuthHandler<S>>>,
}

pub enum PassportStrategies<S> {
    NotAvailable,
    Available(Arc<PassportCell<S>>)
}

pub trait AuthModel {}

pub trait PassportHandler<S>: 'static {
    type Model: AuthModel;
    type Result : Future<Item=Self::Model, Error=Error>;
    fn handle(&self, info: StrategyInfo, req: &HttpRequest<S>) -> Self::Result;
}

impl<F, R, S> PassportHandler<S> for F
where
    F: Fn(StrategyInfo, &HttpRequest<S>) -> FutureResult<R, Error> + 'static,
    R: AuthModel + 'static
{
    type Model = R;
    type Result = FutureResult<Self::Model, Error>;

    fn handle(&self, info: StrategyInfo, req: &HttpRequest<S>) -> Self::Result {
        (self)(info, req)
    }
}

pub(crate) struct WrapperPassportHandler<S, H, R>
where
    H: PassportHandler<S, Model = R>,
    R: AuthModel,
    S: 'static
{
    h: H,
    s: PhantomData<S>
}

impl<S, H, R> WrapperPassportHandler<S, H, R>
    where
        H: PassportHandler<S, Model = R>,
        R: AuthModel,
        S: 'static,
{
    pub fn new(h: H) -> Self {
        WrapperPassportHandler { h, s: PhantomData }
    }
}

pub(crate) trait AuthHandler<S>: 'static{
    fn handle(&self, info: StrategyInfo, req: &HttpRequest<S>, store: bool) -> bool;
}

impl<S, H, R> AuthHandler<S> for WrapperPassportHandler<S, H, R>
where
    H: PassportHandler<S, Model = R>,
    R: AuthModel + 'static,
    S: 'static
{
    fn handle(&self, info: StrategyInfo, req: &HttpRequest<S>, store: bool) -> bool {
        self.h.handle(info, req);

        true
    }
}




impl<S: 'static> Passport<S> {

    pub fn new<F: PassportHandler<S>>(h: F) -> Self {
        Passport {
            inner: PassportInner {
                config: PassportConfig::default(),
                strategies: PassportStrategies::Available(Arc::new(PassportCell(Mutex::new(HashMap::new())))),
                handler: Arc::new(Box::new(WrapperPassportHandler::new(h)))
            },
            finished: false
        }
    }

    pub fn with_strategies_manager(mut self) -> Self {
        self.inner.config.manager_strategies = true;
        self
    }

    pub fn register(self, strategy_name: &str, strategy: Box<PassportStrategy<S>>) -> Self {
        self.inner.strategies.add(strategy_name, strategy);
        self
    }

    pub fn unregister(self, strategy_name: &str) -> Self {
        self.inner.strategies.remove(strategy_name);
        self
    }
}


impl<S: 'static> Middleware<S> for Passport<S> {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        if let Available(ref arc) = self.inner.strategies {
            if self.inner.config.manager_strategies {
                req.extensions_mut().insert(arc.clone());
            }
        }
        Ok(Started::Done)
    }

    fn response(
        &self, req: &HttpRequest<S>, resp: HttpResponse,
    ) -> Result<Response> {
        println!("Middleware main: response");
        if let Some(payload) = req.extensions().get::<Arc<Payload>>() {
            println!("Middleware main: Get gate type successed.");

            let strategy_name: &str = payload._type.as_ref();
//            let store_session = payload.config.store_session;
            let handler = self.inner.handler.clone();
            let req = req.clone();
            return match self.inner.strategies {
                Available(ref arc) => {
                    let guard = arc.0.lock();
                    let strategy = guard.get(strategy_name).expect(format!("Strategy {} not registered", strategy_name).as_ref());
                    //                let req = req.clone();
                    let fut = strategy.extract_info(&req)
                        .map_err(|e| {
                            ErrorUnauthorized(e)
                        })
                        .and_then(move |info|  {
                            handler.handle(info, &req, true);
                            FutOk(resp)
                        });
                    Ok(Response::Future(Box::new(fut)))
                },
                _ => Ok(Response::Done(resp))
            }
        }
        Ok(Response::Done(resp))
    }

    fn finish(&self, req: &HttpRequest<S>, resp: &HttpResponse) -> Finished {
        if self.inner.config.manager_strategies {
            req.extensions_mut().remove::<Arc<PassportCell<S>>>();
        }
        Finished::Done
    }
}

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

    struct User;
    impl AuthModel for User {}

    fn authenticate(info: StrategyInfo, req: &HttpRequest) -> FutureResult<User, Error> {
        FutOk(User)
    }

    #[test]
    fn default_passport_should_run_without_error() {

        let mut srv = test::TestServer::with_factory(|| {
            App::new()
                .middleware(Passport::new(authenticate))
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
                .middleware(Passport::new(authenticate)
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
        request.headers_mut().append(header::AUTHORIZATION, header::HeaderValue::from_static("Basic YWJjOg=="));
        let response  = srv.execute(request.send()).unwrap();
        println!("{}", response.status());
        assert!(response.status().is_success());

    }
//
//    #[test]
//    fn passport_extractor() {
//        let mut srv = test::TestServer::with_factory(|| {
//            App::new()
//                .middleware(Passport::new(|_, _| {
//                    FutOk(())
//                }))
//                .resource("/", |r| {
//                    r.with(|strategies: PassportStrategies<()>| {
//                        assert!(true);
//                        "test"
//                    })
//                })
//        });
//
//        let request = srv.get().uri(srv.url("/")).finish().unwrap();
//        let response = srv.execute(request.send()).unwrap();
//        assert!(response.status().is_success());
//    }
//
//    #[test]
//    fn passport_extractor_dynamic_register() {
//        let mut srv = test::TestServer::with_factory(|| {
//            App::new()
//                .middleware(Passport::new(|_,_|{
//                    FutOk(())
//                }).with_strategies_manager())
//                .resource("/", |r| {
//                    r.with(|mut strategies: PassportStrategies<()>| {
//                        if let Available(_) = strategies {
//                            strategies.add(
//                                "Basic",
//                                Box::new(BasicStrategy{})
//                            );
//                            assert!(strategies.has_strategy("Basic"));
//                        }
//                        "test"
//                    })
//                })
//        });
//
//        let request = srv.get().uri(srv.url("/")).finish().unwrap();
//        let response = srv.execute(request.send()).unwrap();
//        assert!(response.status().is_success());
//    }

}