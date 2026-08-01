use crate::university::DepartmentId;
use sword::web::*;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum StatsError {
	#[http(code = 404, message = "Departamento no encontrado")]
	#[error("Department not found: {0}")]
	DepartmentNotFound(DepartmentId),
}
