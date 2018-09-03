use std::result;
use actix_web::{FromRequest, HttpRequest, HttpResponse, Error, Result, middleware::{Middleware, Response,Started}};
use futures::future::{err as FutErr, ok as FutOk, FutureResult};
use futures::Future;
mod basic;
//mod bearer;
use actix_web::error::ParseError;

pub use self::basic::{BasicStrategy, BasicInfo};


pub enum StrategyInfo {
    Basic(BasicInfo),
    None
}

pub trait PassportStrategy<S> {

    fn extract_info(&self, &HttpRequest<S>,) -> FutureResult<StrategyInfo, ParseError>;
}