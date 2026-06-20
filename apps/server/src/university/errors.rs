use sword::web::*;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum UniversityError {
    // Careers errors
    #[http(code = 404, message = "Carrera no encontrada")]
    #[error("Career not found")]
    CareerNotFound,

    // Departments errors
    #[http(code = 404, message = "Departamento no encontrado")]
    #[error("Department not found")]
    DepartmentNotFound,

    // Faculties errors
    #[http(code = 404, message = "Facultad no encontrada")]
    #[error("Faculty not found")]
    FacultyNotFound,

    // Work Positions errors
    #[http(code = 404, message = "Puesto de trabajo no encontrado")]
    #[error("Work position not found")]
    WorkPositionNotFound,
}
