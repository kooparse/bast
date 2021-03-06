use actix_web::error::{self, Error as ActixError};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Db {
    pub pool: DbPool,
}

impl Db {
    /// Create a new pool.
    pub fn new() -> Self {
        let database_url = dotenv::var("DATABASE_URL")
            .expect("Failed to find database's address in env variable");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(2)
            .build(manager)
            .expect("Failed to create db pool.");

        Self { pool }
    }

    /// This return the database connection
    /// through a pool (wrapped into a Result).
    pub fn conn_pool(&self) -> Result<Conn, ActixError> {
        self.pool.get().map_err(|e| {
            eprintln!("{}", e);
            error::ErrorInternalServerError(e)
        })
    }
}
