use sword::web::*;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum UniversityError {
	#[http(code = 404, message = "La Carrera no fue encontrada")]
	#[error("La Carrera no fue encontrada")]
	CareerNotFound,

	#[http(code = 404, message = "El Departamento no fue encontrado")]
	#[error("El Departamento no fue encontrado")]
	DepartmentNotFound,

	#[http(code = 404, message = "La Facultad no fue encontrada")]
	#[error("La Facultad no fue encontrada")]
	FacultyNotFound,

	#[http(code = 404, message = "El Cargo no fue encontrado")]
	#[error("El Cargo no fue encontrado")]
	WorkPositionNotFound,

	#[http(
		code = 400,
		message = "Se debe especificar un cargo nuevo o un cargo existente"
	)]
	#[error("Se debe especificar un cargo nuevo o un cargo existente")]
	WorkPositionMissing,

	#[http(code = 400, message = "El código de país no es válido")]
	#[error("El código de país no es válido")]
	CountryNotFound(String),

	#[http(
		code = 400,
		message = "La carrera no pertenece al departamento especificado"
	)]
	#[error("La carrera no pertenece al departamento especificado")]
	CareerDepartmentMismatch,
}
