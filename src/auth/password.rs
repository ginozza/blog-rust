use bcrypt::{hash, verify, DEFAULT_COST};
use crate::utils::error::{AppError, AppResult};

pub fn hash_password(password: &str) -> AppResult<String> {
    hash(password, DEFAULT_COST).map_err(|e| {
        log::error!("Error al hashear contraseña: {}", e);
        AppError::InternalServerError("Error al procesar la contraseña".to_string())
    })
}

pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    verify(password, hash).map_err(|e| {
        log::error!("Error al verificar contraseña: {}", e);
        AppError::InternalServerError("Error al verificar la contraseña".to_string())
    })
} 