use sword::web::*;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, HttpError)]
pub enum StatsError {
	#[http(code = 404, message = "Departamento no encontrado")]
	#[error("Department not found: {0}")]
	DepartmentNotFound(Uuid),
}
