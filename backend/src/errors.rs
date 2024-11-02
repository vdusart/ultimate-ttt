use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use thiserror::Error;

use crate::{
    db::DatabaseError,
    observables::grid::errors::{CellError, GridError},
};

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Cell Error: {0}")]
    Cell(CellError),
    #[error("Grid Error: {0}")]
    Grid(GridError),
    #[error("Database Error: {0}")]
    Database(DatabaseError),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl ApplicationError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Cell(error) => error.status_code(),
            Self::Grid(error) => error.status_code(),
            Self::Database(error) => error.status_code(),
        }
    }

    pub fn error_response(&self) -> HttpResponse {
        let response = ErrorResponse {
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(response)
    }
}
