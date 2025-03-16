// Funciones de validación para la aplicación

/// Valida que un string no esté vacío
pub fn is_not_empty(value: &str) -> bool {
    !value.trim().is_empty()
}

/// Valida que un string tenga al menos cierta longitud
pub fn has_min_length(value: &str, min_length: usize) -> bool {
    value.len() >= min_length
}

/// Valida que un email tenga un formato básico válido
pub fn is_valid_email(email: &str) -> bool {
    // Validación básica: contiene @ y al menos un punto después
    email.contains('@') && email.split('@').nth(1).map_or(false, |domain| domain.contains('.'))
} 