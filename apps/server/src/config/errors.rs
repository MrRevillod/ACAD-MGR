use sword::web::*;
use thiserror::Error;

#[derive(Debug, Error, HttpError)]
pub enum ConfigError {
	#[http(code = 404, message = "Configuración del sistema no encontrada")]
	#[error("App config not found")]
	NotFound,

	#[http(code = 400, message = "El valor máximo de JCE debe ser mayor que 0")]
	#[error("JCE max must be greater than 0")]
	InvalidJceMax,
}
