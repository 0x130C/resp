use actix_web::actix::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;
use std::ops::Deref;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub struct DbExecutor(pub PgPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}



pub fn create_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = PgPool::builder()
        .build(manager)
        .expect("Failed to create pool");
    pool
}

impl Deref for DbExecutor {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}