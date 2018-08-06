use validator::Validate;
use diesel;
use diesel::prelude::*;
use actix_web::actix::*;
use db::DbExecutor;
use chrono;
use uuid::Uuid;
use schema::{users, user_session};


#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub actived_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub role: i16,
    pub state: i16,
    pub profile_id: Option<Uuid>
}


#[derive(Deserialize, Validate, Serialize, Debug)]
pub struct UserLogin {
    login: String,
    password: String
}

impl Message for UserLogin {
    type Result = Result<User, diesel::result::Error>;
}

impl Handler<UserLogin> for DbExecutor {
    type Result = Result<User, diesel::result::Error>;

    fn handle(&mut self, msg: UserLogin, _ctx: &mut Self::Context) -> <Self as Handler<UserLogin>>::Result {
        let conn = self.get().unwrap();
        users::table.filter(users::username.eq(msg.login)).get_result::<User>(&conn)
    }
}

#[derive(Deserialize, Queryable, Serialize, Debug)]
pub struct UserLookup {
    #[serde(rename = "token")]
    pub session_id: String
}

impl Message for UserLookup {
    type Result = Result<User, diesel::result::Error>;
}

impl Handler<UserLookup> for DbExecutor {
    type Result = Result<User, diesel::result::Error>;

    fn handle(&mut self, msg: UserLookup, _ctx: &mut Self::Context) -> <Self as Handler<UserLookup>>::Result {
        let conn = self.get().unwrap();

        users::table
            .inner_join(user_session::table)
            .filter(user_session::token.eq(msg.session_id))
            .select(users::all_columns)
            .get_result::<User>(&conn)
    }
}