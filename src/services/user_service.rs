use chrono::Local;
use diesel::result::Error as DieselError;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use crate::config::database::DbConnection;
use crate::models::entities::{User, NewUser, UpdateUser};
use crate::models::dto::{UserDto, CreateUserDto, UpdateUserDto, LoginDto, AuthResponseDto, TokenResponseDto};
use crate::utils::error::{AppError, AppResult};
use crate::auth::{hash_password, verify_password, Claims, create_token};

pub struct UserService;

impl UserService {
    /// Obtiene todos los usuarios
    pub fn get_all_users(conn: &mut DbConnection) -> AppResult<Vec<UserDto>> {
        use crate::db::schema::users::dsl::*;
        
        let users_result = users
            .order(id.asc())
            .load::<User>(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        // Convertir entidades a DTOs
        let user_dtos = users_result.into_iter()
            .map(|user| UserDto {
                id: user.id,
                username: user.username,
                email: user.email,
                created_at: user.created_at,
                updated_at: user.updated_at,
                role: user.role,
            })
            .collect();
        
        Ok(user_dtos)
    }
    
    /// Obtiene un usuario por su ID
    pub fn get_user_by_id(user_id: i32, conn: &mut DbConnection) -> AppResult<UserDto> {
        use crate::db::schema::users::dsl::*;
        
        let user = users
            .filter(id.eq(user_id))
            .first::<User>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Usuario con ID {} no encontrado", user_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        Ok(UserDto {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
            role: user.role,
        })
    }
    
    /// Obtiene un usuario por su email
    pub fn get_user_by_email(email_value: &str, conn: &mut DbConnection) -> AppResult<User> {
        use crate::db::schema::users::dsl::*;
        
        users
            .filter(email.eq(email_value))
            .first::<User>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Usuario con email '{}' no encontrado", email_value)),
                _ => AppError::DatabaseError(e)
            })
    }
    
    /// Crea un nuevo usuario
    pub fn create_user(dto: CreateUserDto, conn: &mut DbConnection) -> AppResult<UserDto> {
        use crate::db::schema::users;
        
        // Validar datos
        if dto.username.trim().is_empty() {
            return Err(AppError::ValidationError("El nombre de usuario no puede estar vacío".to_string()));
        }
        
        if dto.email.trim().is_empty() {
            return Err(AppError::ValidationError("El email no puede estar vacío".to_string()));
        }
        
        if dto.password.trim().is_empty() || dto.password.len() < 6 {
            return Err(AppError::ValidationError("La contraseña debe tener al menos 6 caracteres".to_string()));
        }
        
        // Verificar si ya existe un usuario con el mismo email
        use crate::db::schema::users::dsl::*;
        let email_exists = users
            .filter(email.eq(&dto.email))
            .first::<User>(conn)
            .is_ok();
        
        if email_exists {
            return Err(AppError::ValidationError(format!("Ya existe un usuario con el email '{}'", dto.email)));
        }
        
        // Verificar si ya existe un usuario con el mismo username
        let username_exists = users
            .filter(username.eq(&dto.username))
            .first::<User>(conn)
            .is_ok();
        
        if username_exists {
            return Err(AppError::ValidationError(format!("Ya existe un usuario con el nombre '{}'", dto.username)));
        }
        
        // Hash de la contraseña con bcrypt
        let hashed_password = hash_password(&dto.password)?;
        
        // Crear el usuario
        let now = Local::now().naive_local();
        let new_user = NewUser {
            username: dto.username,
            email: dto.email,
            password_hash: hashed_password,
            created_at: Some(now),
            updated_at: Some(now),
            role: dto.role.unwrap_or_else(|| "user".to_string()),
        };
        
        // Insertar el usuario
        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        Ok(UserDto {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
            role: user.role,
        })
    }
    
    /// Actualiza un usuario existente
    pub fn update_user(user_id: i32, dto: UpdateUserDto, conn: &mut DbConnection) -> AppResult<UserDto> {
        use crate::db::schema::users::dsl::*;
        
        // Verificar que el usuario existe
        let user = users
            .filter(id.eq(user_id))
            .first::<User>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Usuario con ID {} no encontrado", user_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        // Verificar si se está cambiando el email y si el nuevo email ya existe
        if let Some(ref new_email) = dto.email {
            if new_email != &user.email {
                let email_exists = users
                    .filter(email.eq(new_email))
                    .first::<User>(conn)
                    .is_ok();
                
                if email_exists {
                    return Err(AppError::ValidationError(format!("Ya existe un usuario con el email '{}'", new_email)));
                }
            }
        }
        
        // Verificar si se está cambiando el username y si el nuevo username ya existe
        if let Some(ref new_username) = dto.username {
            if new_username != &user.username {
                let username_exists = users
                    .filter(username.eq(new_username))
                    .first::<User>(conn)
                    .is_ok();
                
                if username_exists {
                    return Err(AppError::ValidationError(format!("Ya existe un usuario con el nombre '{}'", new_username)));
                }
            }
        }
        
        // Preparar datos para actualización
        let password_hash_value = if let Some(ref new_password) = dto.password {
            if new_password.is_empty() || new_password.len() < 6 {
                return Err(AppError::ValidationError("La contraseña debe tener al menos 6 caracteres".to_string()));
            }
            // Hash de la contraseña con bcrypt
            Some(hash_password(new_password)?)
        } else {
            None
        };
        
        let update_data = UpdateUser {
            username: dto.username,
            email: dto.email,
            password_hash: password_hash_value,
            updated_at: Some(Local::now().naive_local()),
            role: dto.role,
        };
        
        // Actualizar el usuario
        let updated_user = diesel::update(users.filter(id.eq(user_id)))
            .set(&update_data)
            .get_result::<User>(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        Ok(UserDto {
            id: updated_user.id,
            username: updated_user.username,
            email: updated_user.email,
            created_at: updated_user.created_at,
            updated_at: updated_user.updated_at,
            role: updated_user.role,
        })
    }
    
    /// Elimina un usuario
    pub fn delete_user(user_id: i32, conn: &mut DbConnection) -> AppResult<()> {
        use crate::db::schema::users::dsl::*;
        
        // Verificar que el usuario existe
        let _ = users
            .filter(id.eq(user_id))
            .first::<User>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Usuario con ID {} no encontrado", user_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        // Eliminar el usuario
        let deleted = diesel::delete(users.filter(id.eq(user_id)))
            .execute(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        if deleted == 0 {
            return Err(AppError::NotFound(format!("No se pudo eliminar el usuario con ID {}", user_id)));
        }
        
        Ok(())
    }
    
    /// Autentica un usuario y genera un token JWT
    pub fn login(dto: LoginDto, conn: &mut DbConnection) -> AppResult<AuthResponseDto> {
        // Buscar usuario por email
        let user = Self::get_user_by_email(&dto.email, conn)?;
        
        // Verificar contraseña con bcrypt
        let is_valid = verify_password(&dto.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::AuthenticationError("Credenciales inválidas".to_string()));
        }
        
        // Generar token JWT
        let claims = Claims::new(user.id, user.username.clone(), user.role.clone(), 24); // Token válido por 24 horas
        let token = create_token(claims)
            .map_err(|_| AppError::InternalServerError("Error al generar token".to_string()))?;
        
        // Crear respuesta
        let user_dto = UserDto {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
            role: user.role,
        };
        
        Ok(AuthResponseDto {
            token,
            user: user_dto,
        })
    }
    
    /// Refresca un token JWT
    pub fn refresh_token(user_id: i32, conn: &mut DbConnection) -> AppResult<TokenResponseDto> {
        use crate::db::schema::users::dsl;
        
        // Buscar usuario por ID
        let user = dsl::users
            .filter(dsl::id.eq(user_id))
            .first::<User>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Usuario con ID {} no encontrado", user_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        // Generar nuevo token JWT
        let claims = Claims::new(user.id, user.username, user.role, 24); // Token válido por 24 horas
        let token = create_token(claims)
            .map_err(|_| AppError::InternalServerError("Error al generar token".to_string()))?;
        
        Ok(TokenResponseDto { token })
    }
} 