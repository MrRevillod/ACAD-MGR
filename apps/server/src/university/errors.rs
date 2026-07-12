use sword::web::*;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum UniversityError {
	// Careers errors
	#[http(code = 404, message = "La Carrera no fue encontrada")]
	#[error("Career not found")]
	CareerNotFound,

	// Departments errors
	#[http(code = 404, message = "El Departamento no fue encontrado")]
	#[error("Department not found")]
	DepartmentNotFound,

	// Faculties errors
	#[http(code = 404, message = "La Facultad no fue encontrada")]
	#[error("Faculty not found")]
	FacultyNotFound,

	// Work Positions errors
	#[http(code = 404, message = "El Cargo no fue encontrado")]
	#[error("Work position not found")]
	WorkPositionNotFound,

	#[http(
		code = 400,
		message = "Se debe especificar un cargo nuevo o un cargo existente"
	)]
	#[error("Either a new work position or an existing work position must be specified")]
	WorkPositionMissing,

	// Country errors
	#[http(code = 400, message = "El código de país no es válido")]
	#[error("Country code '{0}' not found")]
	CountryNotFound(String),

	// Career — Department mismatch
	#[http(
		code = 400,
		message = "La carrera no pertenece al departamento especificado"
	)]
	#[error("Career does not belong to the specified department")]
	CareerDepartmentMismatch,
}
