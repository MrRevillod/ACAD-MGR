use sword::web::*;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum AcademicError {
	// Academic category errors
	#[http(code = 404, message = "Categoría académica no encontrada")]
	#[error("Category not found")]
	CategoryNotFound,

	#[http(code = 404, message = "Opción de categoría no encontrada")]
	#[error("Category option not found")]
	CategoryOptionNotFound,

	#[http(
		code = 400,
		message = "La opción de categoría no coincide con la planta académica"
	)]
	#[error("Category option does not match the academic planta")]
	CategoryPlantaMismatch,

	// Academic Option Mismatch errors
	#[http(
		code = 400,
		message = "La opción de categoría no coincide con la categoría académica"
	)]
	#[error("Category option does not match the academic category")]
	CategoryOptionCategoryMismatch,

	#[http(
		code = 400,
		message = "La opción de categoría no coincide con la categoría académica"
	)]
	#[error("Category option does not match the academic category")]
	CategoryOptionMismatch,

	#[http(
		code = 400,
		message = "Las horas de la opción de categoría no coinciden con las horas proporcionadas"
	)]
	#[error("Category option hours do not match the provided hours")]
	CategoryOptionHoursMismatch,

	// Degree errors
	#[http(code = 404, message = "Grado académico no encontrado")]
	#[error("Degree not found")]
	DegreeNotFound,

	// Academic errors
	#[http(code = 409, message = "Ya existe un académico con el mismo RUT")]
	#[error("Academic with the same RUT already exists")]
	AcademicRutAlreadyExists,

	#[http(code = 409, message = "Ya existe un académico con el mismo ORCID")]
	#[error("Academic with the same ORCID already exists")]
	AcademicOrcidAlreadyExists,

	#[http(code = 404, message = "Académico no encontrado")]
	#[error("Academic not found")]
	AcademicNotFound,
}
