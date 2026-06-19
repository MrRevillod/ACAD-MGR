use sqlx::Error as SqlxError;
use std::io::Error as IoError;
use sword::web::*;
use thiserror::Error;

pub type AppResult<T = JsonResponse> = Result<T, AppError>;

#[derive(Debug, Error, HttpError)]
pub enum AppError {
    #[http(code = 500)]
    #[tracing(error)]
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    #[http(code = 500)]
    #[tracing(error)]
    #[error("IO error: {0}")]
    Io(#[from] IoError),

    #[http(code = 500, message = "Internal Server Error")]
    #[error("Internal Error")]
    InternalError,
}
