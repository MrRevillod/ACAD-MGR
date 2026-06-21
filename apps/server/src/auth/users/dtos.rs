use crate::auth::{User, UserId, UserRole};

use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::LazyLock;
use validator::Validate;

static PASSWORD_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[^A-Za-z0-9]).{8,}$").expect("regex inválida")
});

#[derive(Debug, Default, Validate, Deserialize)]
pub struct GetUsersQuery {
    #[validate(length(
        max = 255,
        message = "El atributo 'search' no puede tener más de 255 caracteres"
    ))]
    pub search: Option<String>,
    pub role: Option<UserRole>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUserDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "El atributo 'name' no puede tener más de 255 caracteres"
    ))]
    pub name: String,

    #[validate(email(message = "El atributo 'email' debe ser un correo electrónico válido"))]
    pub email: String,

    #[validate(regex(
    	path = *PASSWORD_REGEX,
     	message = "
	      	La contraseña debe tener al menos 8 caracteres,
	      	incluyendo una letra mayúscula, una letra minúscula, un número y un carácter especial
       	"
    ))]
    pub password: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct UserView {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}

impl From<User> for UserView {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
        }
    }
}
