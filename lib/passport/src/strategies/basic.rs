use super::PassportStrategy;
use super::StrategyInfo;

use actix_web::http::header::{HeaderValue, AUTHORIZATION};
use actix_web::{HttpRequest, Error};
use actix_web::error::ParseError;
use futures::future::{err as FutErr, ok as FutOk, FutureResult};


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
    fn parse_header(&self, header: &HeaderValue) -> Result<StrategyInfo, ParseError> {
        // "Basic *" length
        if header.len() < 7 {
            return Err(ParseError::Invalid);
        }

        let mut parts = header.to_str()?.splitn(2, ' ');
        match parts.next() {
            Some(scheme) if scheme == "Basic" => (),
            _ => return Err(ParseError::MissingScheme),
        }

        let decoded = base64::decode(parts.next().ok_or(ParseError::Invalid)?)?;
        let mut credentials = str::from_utf8(&decoded)?
            .splitn(2, ':');

        let username = credentials.next()
            .ok_or(ParseError::MissingField("username"))
            .map(|username| username.to_string())?;
        let password = credentials.next()
            .ok_or(ParseError::MissingField("password"))
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

    fn extract_info(&self, req: &HttpRequest<S>) -> FutureResult<StrategyInfo, ParseError> {
        let result = req.headers().get(AUTHORIZATION).ok_or(ParseError::Header);
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