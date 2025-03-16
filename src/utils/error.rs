use diesel::result::Error as DieselError;
use thiserror::Error;
use std::io::Error as IoError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error de base de datos: {0}")]
    DatabaseError(#[from] DieselError),
    
    #[error("Error de IO: {0}")]
    IoError(#[from] IoError),
    
    #[error("Recurso no encontrado: {0}")]
    NotFound(String),
    
    #[error("Error de validación: {0}")]
    ValidationError(String),
    
    #[error("Error de autenticación: {0}")]
    AuthenticationError(String),
    
    #[error("Error de autorización: {0}")]
    AuthorizationError(String),
    
    #[error("Error interno del servidor: {0}")]
    InternalServerError(String),
}

pub type AppResult<T> = Result<T, AppError>; 