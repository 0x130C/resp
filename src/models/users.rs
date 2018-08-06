use validator::Validate;
use diesel;
use diesel::prelude::*;
use actix_web::actix::*;
use db::DbExecutor;
use chrono;
use uuid::Uuid;


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
        use schema::users::dsl::*;
        let conn = self.get().unwrap();
        users.filter(username.eq(msg.login)).get_result::<User>(&conn)
    }
}