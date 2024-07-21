use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{ Pool, Postgres };

pub async fn get_pool() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect(&format!("Error connecting to {}", database_url))
}
