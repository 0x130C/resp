use std::io::{Error as IoError, ErrorKind};
use futures::Future;
use futures::future;
use actix_web::{HttpRequest, Error};
use actix_web::middleware::session::RequestSession;

use AppState;

pub type UserAuthenticationResult = Box<Future<Item = User, Error = Error>>;

pub trait UserAuthentication {
    fn is_authenticated(&self) -> bool;
    fn user(&self) -> UserAuthenticationResult;
}
impl UserAuthentication for HttpRequest<AppState> {
    #[inline(always)]
    fn is_authenticated(&self) -> bool {
        match self.session().get::<i32>("uid") {
            Ok(session) => {
                match session {
                    Some(_session_id) => true,
                    None => false
                }
            },

            Err(e) => {
                error!("Error'd when attempting to fetch session data: {:?}", e);
                false
            }
        }
    }

    fn user(&self) -> UserAuthenticationResult {
        match self.session().get::<i32>("uid") {
            Ok(session) => { match session {
                Some(session_id) => {
                    Box::new(self.state().db.send(UserLookup {
                        id: session_id
                    }).from_err().and_then(|res| match res {
                        Ok(user) => Ok(user),
                        Err(err) => {
                            // Temporary because screw matching all these error types
                            let e = IoError::new(ErrorKind::NotFound, format!("{}", err));
                            Err(e.into())
                        }
                    }))
                },

                None => {
                    let e = IoError::new(ErrorKind::NotFound, "User has no session data.");
                    Box::new(future::err(e.into()))
                }
            }},

            Err(e) => {
                error!("Error'd when attempting to fetch session data: {:?}", e);
                Box::new(future::err(e.into()))
            }
        }
    }
}
