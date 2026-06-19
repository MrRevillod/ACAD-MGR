use sword::web::*;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum AcademicError {}
