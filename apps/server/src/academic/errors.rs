use sword::web::*;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum AcademicError {
    #[http(code = 404, message = "Categoría académica no encontrada")]
    #[error("Category not found")]
    CategoryNotFound,

    #[http(code = 404, message = "Opción de categoría no encontrada")]
    #[error("Category option not found")]
    CategoryOptionNotFound,

    #[http(code = 404, message = "Grado académico no encontrado")]
    #[error("Degree not found")]
    DegreeNotFound,

    #[http(code = 404, message = "Académico no encontrado")]
    #[error("Academic not found")]
    AcademicNotFound,
}
