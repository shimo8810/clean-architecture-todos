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

pub fn establish_connection() -> PgTaskRepository {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");

    PgTaskRepository::new(pool)
}
