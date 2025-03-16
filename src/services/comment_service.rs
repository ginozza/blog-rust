use chrono::Local;
use diesel::result::Error as DieselError;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use crate::config::database::DbConnection;
use crate::models::entities::{Comment, NewComment, UpdateComment};
use crate::models::dto::{CommentDto, CreateCommentDto, UpdateCommentDto};
use crate::utils::error::{AppError, AppResult};

pub struct CommentService;

impl CommentService {
    /// Obtiene todos los comentarios
    pub fn get_all_comments(conn: &mut DbConnection) -> AppResult<Vec<CommentDto>> {
        use crate::db::schema::comments::dsl::*;
        
        let comments_result = comments
            .order(created_at.desc())
            .load::<Comment>(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        // Convertir entidades a DTOs
        let comment_dtos = comments_result.into_iter()
            .map(|comment| CommentDto {
                id: comment.id,
                post_id: comment.post_id,
                user_id: comment.user_id,
                author_name: comment.author_name,
                author_email: comment.author_email,
                content: comment.content,
                created_at: comment.created_at,
                updated_at: comment.updated_at,
            })
            .collect();
        
        Ok(comment_dtos)
    }

    /// Obtiene todos los comentarios de un post
    pub fn get_comments_by_post(post_id_param: i32, conn: &mut DbConnection) -> AppResult<Vec<CommentDto>> {
        use crate::db::schema::comments::dsl::*;
        
        let comments_result = comments
            .filter(post_id.eq(post_id_param))
            .order(created_at.desc())
            .load::<Comment>(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        // Convertir entidades a DTOs
        let comment_dtos = comments_result.into_iter()
            .map(|comment| CommentDto {
                id: comment.id,
                post_id: comment.post_id,
                user_id: comment.user_id,
                author_name: comment.author_name,
                author_email: comment.author_email,
                content: comment.content,
                created_at: comment.created_at,
                updated_at: comment.updated_at,
            })
            .collect();
        
        Ok(comment_dtos)
    }
    
    /// Alias para get_comments_by_post para mantener compatibilidad
    pub fn get_comments_by_post_id(post_id: i32, conn: &mut DbConnection) -> AppResult<Vec<CommentDto>> {
        Self::get_comments_by_post(post_id, conn)
    }
    
    /// Obtiene un comentario por su ID
    pub fn get_comment_by_id(comment_id: i32, conn: &mut DbConnection) -> AppResult<CommentDto> {
        use crate::db::schema::comments::dsl::*;
        
        let comment = comments
            .filter(id.eq(comment_id))
            .first::<Comment>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Comentario con ID {} no encontrado", comment_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        Ok(CommentDto {
            id: comment.id,
            post_id: comment.post_id,
            user_id: comment.user_id,
            author_name: comment.author_name,
            author_email: comment.author_email,
            content: comment.content,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
        })
    }
    
    /// Crea un nuevo comentario
    pub fn create_comment(dto: CreateCommentDto, conn: &mut DbConnection) -> AppResult<CommentDto> {
        use crate::db::schema::comments;
        use crate::db::schema::posts::dsl::*;
        
        // Validar datos
        if dto.content.trim().is_empty() {
            return Err(AppError::ValidationError("El contenido no puede estar vacío".to_string()));
        }
        
        // Verificar que el post existe
        let post_exists = posts
            .filter(id.eq(dto.post_id))
            .first::<crate::models::entities::Post>(conn)
            .is_ok();
        
        if !post_exists {
            return Err(AppError::ValidationError(format!("El post con ID {} no existe", dto.post_id)));
        }
        
        // Variables para almacenar el nombre y email del autor
        let mut author_name = dto.author_name;
        let mut author_email = dto.author_email;
        
        // Verificar que si se proporciona un user_id, el usuario existe
        if let Some(user_id_value) = dto.user_id {
            use crate::db::schema::users::dsl::*;
            
            // Buscar el usuario para obtener su nombre y email
            match users
                .filter(id.eq(user_id_value))
                .first::<crate::models::entities::User>(conn) {
                Ok(user) => {
                    // Si el comentario es de un usuario registrado, usar su nombre de usuario
                    author_name = Some(user.username);
                    // Si no se proporcionó un email específico, usar el del usuario
                    if author_email.is_none() {
                        author_email = Some(user.email);
                    }
                },
                Err(_) => {
                    return Err(AppError::ValidationError(format!("El usuario con ID {} no existe", user_id_value)));
                }
            }
        } else {
            // Si no hay user_id, debe haber author_name
            if author_name.is_none() || author_name.as_ref().unwrap().trim().is_empty() {
                return Err(AppError::ValidationError("Se requiere un nombre de autor para comentarios anónimos".to_string()));
            }
        }
        
        // Crear el comentario
        let now = Local::now().naive_local();
        let new_comment = NewComment {
            post_id: dto.post_id,
            user_id: dto.user_id,
            author_name,
            author_email,
            content: dto.content,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Insertar el comentario
        let comment = diesel::insert_into(comments::table)
            .values(&new_comment)
            .get_result::<Comment>(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        Ok(CommentDto {
            id: comment.id,
            post_id: comment.post_id,
            user_id: comment.user_id,
            author_name: comment.author_name,
            author_email: comment.author_email,
            content: comment.content,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
        })
    }
    
    /// Actualiza un comentario existente
    pub fn update_comment(comment_id: i32, dto: UpdateCommentDto, conn: &mut DbConnection) -> AppResult<CommentDto> {
        use crate::db::schema::comments::dsl::*;
        
        // Verificar que el comentario existe
        let _comment = comments
            .filter(id.eq(comment_id))
            .first::<Comment>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Comentario con ID {} no encontrado", comment_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        // Validar datos
        if let Some(ref content_value) = dto.content {
            if content_value.is_empty() {
                return Err(AppError::ValidationError("El contenido no puede estar vacío".to_string()));
            }
        }
        
        // Preparar datos para actualización
        let update_data = UpdateComment {
            content: dto.content,
            updated_at: Some(Local::now().naive_local()),
        };
        
        // Actualizar el comentario
        let updated_comment = diesel::update(comments.filter(id.eq(comment_id)))
            .set(&update_data)
            .get_result::<Comment>(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        Ok(CommentDto {
            id: updated_comment.id,
            post_id: updated_comment.post_id,
            user_id: updated_comment.user_id,
            author_name: updated_comment.author_name,
            author_email: updated_comment.author_email,
            content: updated_comment.content,
            created_at: updated_comment.created_at,
            updated_at: updated_comment.updated_at,
        })
    }
    
    /// Elimina un comentario
    pub fn delete_comment(comment_id: i32, conn: &mut DbConnection) -> AppResult<()> {
        use crate::db::schema::comments::dsl::*;
        
        // Verificar que el comentario existe
        let _ = comments
            .filter(id.eq(comment_id))
            .first::<Comment>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Comentario con ID {} no encontrado", comment_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        // Eliminar el comentario
        let deleted = diesel::delete(comments.filter(id.eq(comment_id)))
            .execute(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        if deleted == 0 {
            return Err(AppError::NotFound(format!("No se pudo eliminar el comentario con ID {}", comment_id)));
        }
        
        Ok(())
    }
} 