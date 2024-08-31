use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{ Pool, Postgres };

pub async fn get_pool(database_url: &String) -> Pool<Postgres> {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect(&format!("Error connecting to {}", database_url))
}
