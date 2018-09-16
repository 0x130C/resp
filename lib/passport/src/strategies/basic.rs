use super::PassportStrategy;
use super::StrategyInfo;

use actix_web::http::header::{HeaderValue, AUTHORIZATION};
use actix_web::{HttpRequest, Error};
use actix_web::error::ParseError;
use futures::future::{err as FutErr, ok as FutOk, FutureResult};
use super::super::error::ExtractError;
use base64;
use std::str;

pub struct BasicStrategy;

pub struct BasicInfo {
    pub username: String,
    pub password: Option<String>
}

impl Default for BasicInfo {
    fn default() -> Self {
        BasicInfo {
            username: "".to_owned(),
            password: None
        }
    }
}



impl BasicStrategy {
    fn parse_header(&self, header: &HeaderValue) -> Result<StrategyInfo, ExtractError> {
        // "Basic *" length
        if header.len() < 7 {
            return Err(ExtractError::Invalid);
        }

        let mut parts = header.to_str()?.splitn(2, ' ');
        match parts.next() {
            Some(scheme) if scheme == "Basic" => (),
            _ => return Err(ExtractError::MissingScheme),
        }

        let decoded = base64::decode(parts.next().ok_or(ExtractError::Invalid)?)?;
        let mut credentials = str::from_utf8(&decoded)?
            .splitn(2, ':');

        let username = credentials.next()
            .ok_or(ExtractError::MissingField("username"))
            .map(|username| username.to_string())?;
        let password = credentials.next()
            .ok_or(ExtractError::MissingField("password"))
            .map(|password| {
                if password.is_empty() {
                    None
                } else {
                    Some(password.to_string())
                }
            })?;

        Ok(StrategyInfo::Basic(BasicInfo {
            username,
            password,
        }))
    }
}


impl<S: 'static> PassportStrategy<S> for BasicStrategy {

    fn extract_info(&self, req: &HttpRequest<S>) -> FutureResult<StrategyInfo, ExtractError> {
        let result = req.headers().get(AUTHORIZATION).ok_or(ExtractError::Invalid);
        match result {
            Ok(header) => {
                match self.parse_header(header) {
                    Ok(info) => FutOk(info),
                    Err(err) => FutErr(err)
                }
            },
            Err(err) => {
                FutErr(err)
            }
        }

    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::Passport;
    use futures::Future;
    use actix_web::{App, test, Result, HttpResponse};
    use actix_web::{http::header, middleware::{Middleware, Started, Response, Finished}};

    struct MiddlewareTest01;

    impl<S> Middleware<S> for MiddlewareTest01 {
        fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
            println!("Middleware start 01");
            Ok(Started::Done)
        }
        fn response(&self, req: &HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
            println!("Middleware response 01");
            Ok(Response::Done(resp))
        }

        /// Method is called after body stream get sent to peer.
        fn finish(&self, req: &HttpRequest<S>, resp: &HttpResponse) -> Finished {
            println!("Middleware finish 01");
            Finished::Done
        }
    }

    struct MiddlewareTest02;

    impl<S> Middleware<S> for MiddlewareTest02 {
        fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
            println!("Middleware start 02");
            Ok(Started::Done)
        }
        fn response(&self, req: &HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
            println!("Middleware response 02");
            Ok(Response::Done(resp))
        }

        /// Method is called after body stream get sent to peer.
        fn finish(&self, req: &HttpRequest<S>, resp: &HttpResponse) -> Finished {
            println!("Middleware finish 02");
            Finished::Done
        }
    }
    #[test]
    fn test_parser_header() {
        let header = header::HeaderValue::from_static("Basic YWJjOg==");
        let strategy = BasicStrategy{};
        let result = strategy.parse_header(&header).unwrap();
        match result {
            StrategyInfo::Basic(info) => {
                assert_eq!(info.username, "abc");
                assert_eq!(info.password, None)
            },
            _ => {
                assert!(false)
            }
        }
    }

//    #[test]
//    fn test_basic_strategy_success() {
//        let mut srv = test::TestServer::with_factory(|| {
//            App::new()
//                .middleware({ MiddlewareTest01 })
//                .middleware(Passport::new(|info, req|{
//                    if let StrategyInfo::Basic(info) = info {
//                        assert_eq!(info.username, "Aladdin");
//                        assert_eq!(info.password, None);
//                    }
//                    FutOk(())
//                })
//                    .register(
//                        "Basic",
//                        Box::new(BasicStrategy{})
//                    )
//                )
//                .resource("/", |r| {
//                    r.middleware(MiddlewareTest02);
//                    r.f(|req| {
//                        "test"
//                    })
//                })
//        });
//        let mut request = srv.get().uri(srv.url("/")).finish().unwrap();
//        request.headers_mut().append(header::AUTHORIZATION, header::HeaderValue::from_static("Basic QWxhZGRpbjo="));
//        let response = srv.execute(request.send()).unwrap();
//
//    }
}