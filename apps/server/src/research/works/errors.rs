use sword::web::*;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum WorksError {
    #[http(code = 404, message = "Académico no encontrado")]
    #[error("Academic not found")]
    AcademicNotFound,

    #[http(code = 422, message = "El académico no tiene ORCID asociado")]
    #[error("Academic has no ORCID")]
    AcademicWithoutOrcid,

    #[http(code = 404, message = "Obra no encontrada")]
    #[error("Work not found")]
    WorkNotFound,

    #[http(code = 502, message = "Error al consultar OpenAlex")]
    #[error("OpenAlex API error: {0}")]
    OpenAlexError(String),
}
