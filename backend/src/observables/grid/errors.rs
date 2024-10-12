use actix_web::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CellError {
    #[error("Impossible to load cell from bits")]
    Load(),

    #[error("Cell already used")]
    AlreadyUsed(),
}

impl CellError {
    pub fn status_code(&self) -> StatusCode {
        match *self {
            Self::Load() => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AlreadyUsed() => StatusCode::BAD_REQUEST,
        }
    }
}


#[derive(Debug, Error)]
pub enum GridError {
    #[error("Invalid grid bytes string lenght: {0}")]
    InvalidLength(usize),

    #[error("Invalid bytes string")]
    InvalidBytesString(),

    #[error("Invalid depth: {0}")]
    InvalidDepth(u8),

    #[error("")]
    NoSubgrid(),

    #[error("Invalid play position: {0}")]
    InvalidPosition(String),
}

impl GridError {
    pub fn status_code(&self) -> StatusCode {
        match *self {
            Self::InvalidLength(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidBytesString() => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidDepth(_) => StatusCode::BAD_REQUEST,
            Self::NoSubgrid() => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidPosition(_) => StatusCode::BAD_REQUEST,
        }
    }
}
