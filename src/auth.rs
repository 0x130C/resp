use futures::future::{err as FutErr, ok as FutOk, FutureResult};
use futures::Future;
use actix_web::error::{Error, ResponseError, Result};
use actix_web::HttpRequest;
pub struct RAuthBuilder {
    services: Vec<String>
}

pub struct RAuth {
    inner: RAuthInner

}

struct RAuthInner {
    services: Vec<String>
}

impl RAuthBuilder {
    pub fn new() -> Self {
        RAuthBuilder{
            services: vec![]
        }
    }

    pub fn link_to(mut self, name: &str) -> Self {
        self.services.push(name.to_owned());
        self
    }

    pub fn finish(&self) -> RAuth {
        let inner = RAuthInner {
            services: self.services.clone()
        };
        RAuth { inner }
    }
}

pub enum RError {

}

pub type RResult<T> = Result<T, RError>;


pub trait RAuthImpl {
    type User;
    type AuthFuture: Future<Item = Self::User, Error = Error>;

    fn authentice(self, req: &mut HttpRequest, config: Self::Config) -> RResult<Self::User>;

}


#[cfg(test)]
mod test {

    #[test]
    fn test_basic_authentication() {
        assert_eq!(1, 1)
    }
}