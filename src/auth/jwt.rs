use actix_web::{error::ErrorUnauthorized, Error as ActixError};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // Subject (ID del usuario)
    pub exp: i64,           // Tiempo de expiración
    pub iat: i64,           // Tiempo de emisión
    pub role: String,       // Rol del usuario (admin, user, etc.)
    pub username: String,   // Nombre de usuario
}

impl Claims {
    pub fn new(user_id: i32, username: String, role: String, expiration_hours: i64) -> Self {
        let now = Utc::now();
        Claims {
            sub: user_id.to_string(),
            exp: (now + Duration::hours(expiration_hours)).timestamp(),
            iat: now.timestamp(),
            role,
            username,
        }
    }
}

pub fn create_token(claims: Claims) -> Result<String, ActixError> {
    let secret = get_secret();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        log::error!("Error al crear token JWT: {}", e);
        ErrorUnauthorized("Error al crear token")
    })
}

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, ActixError> {
    let secret = get_secret();
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        log::error!("Error al validar token JWT: {}", e);
        ErrorUnauthorized("Token inválido o expirado")
    })
}

fn get_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| {
        log::warn!("JWT_SECRET no está configurada, usando valor por defecto (inseguro)");
        "secret_muy_secreto_para_desarrollo_no_usar_en_produccion".to_string()
    })
} 