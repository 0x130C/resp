use actix_web::{HttpRequest, FromRequest, Error};
use models::users::User;
use serde::de::{self, DeserializeOwned};
use futures::future::{self, Future, FutureResult, result};

impl<S> FromRequest<S> for User
    where S: 'static
{
    type Config = ();
    type Result = FutureResult<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        let req = req.clone();
        if req.is_authenticated() {
            req.user().from_err().and_then(move |res| match res {
                Ok(user) => {
                    return future::Ok(user);
                },
                Err(e) => future::Err(e)
            })
        }
    }
}