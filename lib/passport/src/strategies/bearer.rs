use super::PassportStrategy;
use super::ParseError;
use super::User;
use actix_web::HttpRequest;


pub struct BearerStrategy {
    realm: String,
    scope: Vec<String>
}

impl<S> PassportStrategy<S> for BearerStrategy {

    fn authenticate(&self, req: &HttpRequest<S>) -> Result<User, ParseError> {
        Ok(User)
    }
}