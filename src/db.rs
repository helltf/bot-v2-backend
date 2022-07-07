use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection, State};
use diesel::PgConnection;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome};
use rocket::Request;
use std::env;
use std::ops::Deref;

type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_PgPool() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    PgPool::new(manager).expect("db PgPool")
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'r> FromRequest<'r> for DbConn {
    type Error = ();

    fn from_request(request: &'r Request<'_>) -> Outcome<DbConn, Self::Error> {
        return Outcome::Success(DbConn(conn));
    }
}
impl Deref for DbConn {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
