use crate::{
	academic::AcademicError, auth::AuthError, research::WorksError, university::UniversityError,
};

use std::io::Error as IoError;
use sword::web::*;
use thiserror::Error;
use toasty::Error as DatabaseError;

pub type AppResult<T = JsonResponse> = Result<T, AppError>;

#[derive(Debug, Error, HttpError)]
pub enum AppError {
	#[http(transparent)]
	#[error(transparent)]
	Auth(#[from] AuthError),

	#[http(transparent)]
	#[error(transparent)]
	Academic(#[from] AcademicError),

	#[http(transparent)]
	#[error(transparent)]
	University(#[from] UniversityError),

	#[http(transparent)]
	#[error(transparent)]
	Research(#[from] WorksError),

	#[http(code = 500)]
	#[tracing(error)]
	#[error("Database error: {0}")]
	Database(#[from] DatabaseError),

	#[http(code = 500)]
	#[tracing(error)]
	#[error("IO error: {0}")]
	Io(#[from] IoError),

	#[http(code = 500)]
	#[tracing(error)]
	#[error("CSV error: {0}")]
	CsvError(#[from] csv::Error),

	#[http(code = 500, message = "Internal Server Error")]
	#[error("Internal Error")]
	InternalError,

	#[http(code = 500, message = "Internal Server Error")]
	#[error("Jiff time error: {0}")]
	Time(#[from] jiff::Error),
}
