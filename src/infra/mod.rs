pub mod error;
pub mod model;
pub mod repository;
pub mod schema;

pub use repository::DbPool;
pub use repository::PgTaskRepository;

use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub fn establish_connection(database_url: &str) -> PgTaskRepository {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");

    PgTaskRepository::new(pool)
}
