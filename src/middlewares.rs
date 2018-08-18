use actix_web::middleware::{Middleware};
use actix_web::{HttpRequest};

pub struct Passport;


impl Middleware<S> for Passport {
    fn start(&self, req: &mut HttpRequest<S>) -> Result {

    }
}