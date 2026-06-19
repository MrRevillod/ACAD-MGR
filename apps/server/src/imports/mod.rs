use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, EnumIter)]
pub enum Column {
    #[strum(serialize = "RUT")]
    Rut,

    #[strum(serialize = "APELLIDO PATERNO")]
    ApellidoPaterno,

    #[strum(serialize = "APELLIDO MATERNO")]
    ApellidoMaterno,

    #[strum(serialize = "NOMBRES")]
    Nombres,

    #[strum(serialize = "SEXO")]
    Sexo,

    #[strum(serialize = "FECHA DE NACIMIENTO")]
    FechaNacimiento,

    #[strum(serialize = "PAÍS DE NACIONALIDAD")]
    Nacionalidad,

    #[strum(serialize = "FECHA DE INGRESO")]
    FechaIngreso,

    #[strum(serialize = "CARGO ACAD")]
    CargoAcad,

    #[strum(serialize = "FACULTAD")]
    Facultad,

    #[strum(serialize = "DEPARTAMENTO")]
    Departamento,

    #[strum(serialize = "CARRERA")]
    Carrera,

    #[strum(serialize = "JORNADA UCT HORAS")]
    JornadaUctHoras,

    #[strum(serialize = "JCE")]
    Jce,

    #[strum(serialize = "PLANTA")]
    Planta,

    #[strum(serialize = "CATEGORIA")]
    Categoria,

    #[strum(serialize = "OPCION")]
    Opcion,

    #[strum(serialize = "HRS DD CATEGORIA/OPCION")]
    HrsDdCategoriaOpcion,

    #[strum(serialize = "HRS DD DESC PROM ANUAL")]
    HrsDdDescPromAnual,

    #[strum(serialize = "CIUDAD")]
    Ciudad,

    #[strum(serialize = "TITULO")]
    Titulo,

    #[strum(serialize = "UNIVERSIDAD (I)")]
    UniversidadI,

    #[strum(serialize = "PAIS (I)")]
    PaisI,

    #[strum(serialize = "FECHA (I)")]
    FechaI,

    #[strum(serialize = "TITULO/GRADO")]
    TituloGrado,

    #[strum(serialize = "UNIVERSIDAD (II)")]
    UniversidadII,

    #[strum(serialize = "PAIS (II)")]
    PaisII,

    #[strum(serialize = "FECHA (II)")]
    FechaII,

    #[strum(serialize = "CORREO")]
    Correo,

    #[strum(serialize = "ORCID")]
    Orcid,
}

impl Column {
    pub fn normalize(&self) -> String {
        match self {
            Column::Rut => "rut".to_string(),
            Column::ApellidoPaterno => "paternal_surname".to_string(),
            Column::ApellidoMaterno => "maternal_surname".to_string(),
            Column::Nombres => "names".to_string(),
            Column::Sexo => "sex".to_string(),
            Column::FechaNacimiento => "birth_date".to_string(),
            Column::Nacionalidad => "nationality".to_string(),
            Column::FechaIngreso => "joined_at".to_string(),
            Column::CargoAcad => "work_position_name".to_string(),
            Column::Facultad => "faculty_name".to_string(),
            Column::Departamento => "department_name".to_string(),
            Column::Carrera => "career_name".to_string(),
            Column::JornadaUctHoras => "uct_working_hours".to_string(),
            Column::Jce => "jce".to_string(),
            Column::Planta => "planta".to_string(),
            Column::Categoria => "academic_category".to_string(),
            Column::Opcion => "academic_option".to_string(),
            Column::HrsDdCategoriaOpcion => "acad_category_hours".to_string(),
            Column::HrsDdDescPromAnual => "annual_discount_hours".to_string(),
            Column::Ciudad => "city".to_string(),
            Column::Titulo => "base_degree".to_string(),
            Column::UniversidadI => "base_degree_university".to_string(),
            Column::PaisI => "base_degree_university_country".to_string(),
            Column::FechaI => "base_degree_obtained_at".to_string(),
            Column::TituloGrado => "advanced_degree".to_string(),
            Column::UniversidadII => "advanced_degree_university".to_string(),
            Column::PaisII => "advanced_degree_university_country".to_string(),
            Column::FechaII => "advanced_degree_obtained_at".to_string(),
            Column::Correo => "email".to_string(),
            Column::Orcid => "orcid".to_string(),
        }
    }
}
