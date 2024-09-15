use std::time::Duration;

use actix_web::http::StatusCode;
use sqlx::postgres::PgPoolOptions;
use sqlx::{ Pool, Postgres };
use thiserror::Error;


#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Impossible to insert: {0}")]
    Insert(String),
    #[error("{0} not found.")]
    NotFound(String),
}

impl DatabaseError {
    pub fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn get_pool(database_url: &String) -> Pool<Postgres> {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect(&format!("Error connecting to {}", database_url))
}
