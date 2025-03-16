use chrono::Local;
use diesel::result::Error as DieselError;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::Connection;
use crate::config::database::DbConnection;
use crate::models::entities::{NewPost, UpdatePost};
use crate::models::dto::{PostDto, PostDetailDto, CreatePostDto, UpdatePostDto};
use crate::repositories::PostRepository;
use crate::utils::error::{AppError, AppResult};
use crate::utils::slug::slugify;

pub struct PostService;

impl PostService {
    /// Obtiene todos los posts
    pub fn get_all_posts(conn: &mut DbConnection) -> AppResult<Vec<PostDto>> {
        let posts = PostRepository::find_all(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        // Convertir entidades a DTOs
        let post_dtos = posts.into_iter()
            .map(|post| PostDto {
                id: post.id,
                title: post.title,
                slug: post.slug,
                body: post.body,
                created_at: post.created_at,
                updated_at: post.updated_at,
            })
            .collect();
        
        Ok(post_dtos)
    }
    
    /// Obtiene un post por su ID
    pub fn get_post_by_id(post_id: i32, conn: &mut DbConnection) -> AppResult<PostDto> {
        let post = PostRepository::find_by_id(post_id, conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Post con ID {} no encontrado", post_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        Ok(PostDto {
            id: post.id,
            title: post.title,
            slug: post.slug,
            body: post.body,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
    }
    
    /// Obtiene un post por su slug
    pub fn get_post_by_slug(slug: &str, conn: &mut DbConnection) -> AppResult<PostDto> {
        let post = PostRepository::find_by_slug(slug, conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Post con slug '{}' no encontrado", slug)),
                _ => AppError::DatabaseError(e)
            })?;
        
        Ok(PostDto {
            id: post.id,
            title: post.title,
            slug: post.slug,
            body: post.body,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
    }
    
    /// Obtiene un post con sus categorías y comentarios
    pub fn get_post_detail(post_id: i32, conn: &mut DbConnection) -> AppResult<PostDetailDto> {
        // Obtener el post
        let post = PostRepository::find_by_id(post_id, conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Post con ID {} no encontrado", post_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        // Obtener categorías del post
        let categories = PostRepository::get_categories(post_id, conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        // Convertir categorías a DTOs
        let category_dtos = categories.into_iter()
            .map(|category| crate::models::dto::CategoryDto {
                id: category.id,
                name: category.name,
                slug: category.slug,
                description: category.description,
                created_at: category.created_at,
                updated_at: category.updated_at,
            })
            .collect();
        
        // Obtener comentarios del post (pendiente de implementar el repositorio de comentarios)
        // Por ahora, devolvemos una lista vacía
        let comment_dtos = Vec::new();
        
        Ok(PostDetailDto {
            id: post.id,
            title: post.title,
            slug: post.slug,
            body: post.body,
            created_at: post.created_at,
            updated_at: post.updated_at,
            categories: category_dtos,
            comments: comment_dtos,
        })
    }
    
    /// Crea un nuevo post
    pub fn create_post(dto: CreatePostDto, conn: &mut DbConnection) -> AppResult<PostDto> {
        // Validar datos
        if dto.title.trim().is_empty() {
            return Err(AppError::ValidationError("El título no puede estar vacío".to_string()));
        }
        
        if dto.body.trim().is_empty() {
            return Err(AppError::ValidationError("El contenido no puede estar vacío".to_string()));
        }
        
        // Generar slug si no se proporciona
        let slug = match dto.slug {
            Some(s) if !s.trim().is_empty() => s,
            _ => slugify(&dto.title),
        };
        
        // Verificar si ya existe un post con el mismo slug
        match PostRepository::find_by_slug(&slug, conn) {
            Ok(_) => return Err(AppError::ValidationError(format!("Ya existe un post con el slug '{}'", slug))),
            Err(DieselError::NotFound) => {}, // Es lo que queremos, que no exista
            Err(e) => return Err(AppError::DatabaseError(e)),
        }
        
        // Crear el post
        let now = Local::now().naive_local();
        let new_post = NewPost {
            title: dto.title,
            body: dto.body,
            slug,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Insertar el post
        let post = if let Some(category_ids) = dto.category_ids {
            if !category_ids.is_empty() {
                // Insertar con categorías
                PostRepository::create_with_categories(
                    &new_post.title,
                    &new_post.body,
                    Some(&new_post.slug),
                    &category_ids,
                    conn
                )
                .map_err(|e| AppError::DatabaseError(e))?
            } else {
                // Insertar sin categorías
                PostRepository::create(&new_post, conn)
                    .map_err(|e| AppError::DatabaseError(e))?
            }
        } else {
            // Insertar sin categorías
            PostRepository::create(&new_post, conn)
                .map_err(|e| AppError::DatabaseError(e))?
        };
        
        Ok(PostDto {
            id: post.id,
            title: post.title,
            slug: post.slug,
            body: post.body,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
    }
    
    /// Actualiza un post existente
    pub fn update_post(post_id: i32, dto: UpdatePostDto, conn: &mut DbConnection) -> AppResult<PostDto> {
        // Verificar que el post existe
        let post = PostRepository::find_by_id(post_id, conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Post con ID {} no encontrado", post_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        // Verificar si se está cambiando el slug y si el nuevo slug ya existe
        if let Some(ref new_slug) = dto.slug {
            if new_slug != &post.slug {
                match PostRepository::find_by_slug(new_slug, conn) {
                    Ok(_) => return Err(AppError::ValidationError(format!("Ya existe un post con el slug '{}'", new_slug))),
                    Err(DieselError::NotFound) => {}, // Es lo que queremos, que no exista
                    Err(e) => return Err(AppError::DatabaseError(e)),
                }
            }
        }
        
        // Preparar datos para actualización
        let update_data = UpdatePost {
            title: dto.title,
            body: dto.body,
            slug: dto.slug,
            updated_at: Some(Local::now().naive_local()),
        };
        
        // Actualizar el post
        let updated_post = match &dto.category_ids {
            Some(category_ids) => {
                // Actualizar con categorías
                PostRepository::update_with_categories(
                    post_id,
                    update_data.title.as_deref(),
                    update_data.body.as_deref(),
                    update_data.slug.as_deref(),
                    Some(category_ids),
                    conn
                )
                .map_err(|e| AppError::DatabaseError(e))?
            },
            None => {
                // Actualizar sin categorías
                PostRepository::update(post_id, &update_data, conn)
                    .map_err(|e| AppError::DatabaseError(e))?
            }
        };
        
        Ok(PostDto {
            id: updated_post.id,
            title: updated_post.title,
            slug: updated_post.slug,
            body: updated_post.body,
            created_at: updated_post.created_at,
            updated_at: updated_post.updated_at,
        })
    }
    
    /// Elimina un post
    pub fn delete_post(post_id: i32, conn: &mut DbConnection) -> AppResult<()> {
        // Verificar que el post existe
        let _ = PostRepository::find_by_id(post_id, conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Post con ID {} no encontrado", post_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        // Eliminar el post
        let deleted = PostRepository::delete(post_id, conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        if deleted == 0 {
            return Err(AppError::NotFound(format!("No se pudo eliminar el post con ID {}", post_id)));
        }
        
        Ok(())
    }
} 