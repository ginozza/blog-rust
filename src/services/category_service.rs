use chrono::Local;
use diesel::result::Error as DieselError;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::Connection;
use crate::config::database::DbConnection;
use crate::models::entities::{Category, NewCategory, UpdateCategory};
use crate::models::dto::{CategoryDto, CreateCategoryDto, UpdateCategoryDto};
use crate::utils::error::{AppError, AppResult};
use crate::utils::slug::slugify;

pub struct CategoryService;

impl CategoryService {
    /// Obtiene todas las categorías
    pub fn get_all_categories(conn: &mut DbConnection) -> AppResult<Vec<CategoryDto>> {
        use crate::db::schema::categories::dsl::*;
        
        let categories_result = categories
            .order(name.asc())
            .load::<Category>(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        // Convertir entidades a DTOs
        let category_dtos = categories_result.into_iter()
            .map(|category| CategoryDto {
                id: category.id,
                name: category.name,
                slug: category.slug,
                description: category.description,
                created_at: category.created_at,
                updated_at: category.updated_at,
            })
            .collect();
        
        Ok(category_dtos)
    }
    
    /// Obtiene una categoría por su ID
    pub fn get_category_by_id(category_id: i32, conn: &mut DbConnection) -> AppResult<CategoryDto> {
        use crate::db::schema::categories::dsl::*;
        
        let category = categories
            .filter(id.eq(category_id))
            .first::<Category>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Categoría con ID {} no encontrada", category_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        Ok(CategoryDto {
            id: category.id,
            name: category.name,
            slug: category.slug,
            description: category.description,
            created_at: category.created_at,
            updated_at: category.updated_at,
        })
    }
    
    /// Obtiene una categoría por su slug
    pub fn get_category_by_slug(category_slug: &str, conn: &mut DbConnection) -> AppResult<CategoryDto> {
        use crate::db::schema::categories::dsl::*;
        
        let category = categories
            .filter(slug.eq(category_slug))
            .first::<Category>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Categoría con slug '{}' no encontrada", category_slug)),
                _ => AppError::DatabaseError(e)
            })?;
        
        Ok(CategoryDto {
            id: category.id,
            name: category.name,
            slug: category.slug,
            description: category.description,
            created_at: category.created_at,
            updated_at: category.updated_at,
        })
    }
    
    /// Crea una nueva categoría
    pub fn create_category(dto: CreateCategoryDto, conn: &mut DbConnection) -> AppResult<CategoryDto> {
        use crate::db::schema::categories;
        
        // Validar datos
        if dto.name.trim().is_empty() {
            return Err(AppError::ValidationError("El nombre no puede estar vacío".to_string()));
        }
        
        // Generar slug si no se proporciona
        let slug_str = match dto.slug {
            Some(s) if !s.trim().is_empty() => s,
            _ => slugify(&dto.name),
        };
        
        // Verificar si ya existe una categoría con el mismo slug
        use crate::db::schema::categories::dsl::*;
        let slug_exists = categories
            .filter(slug.eq(&slug_str))
            .first::<Category>(conn)
            .is_ok();
        
        if slug_exists {
            return Err(AppError::ValidationError(format!("Ya existe una categoría con el slug '{}'", slug_str)));
        }
        
        // Crear la categoría
        let now = Local::now().naive_local();
        let new_category = NewCategory {
            name: dto.name,
            slug: slug_str,
            description: dto.description,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Insertar la categoría
        let category = diesel::insert_into(categories::table)
            .values(&new_category)
            .get_result::<Category>(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        Ok(CategoryDto {
            id: category.id,
            name: category.name,
            slug: category.slug,
            description: category.description,
            created_at: category.created_at,
            updated_at: category.updated_at,
        })
    }
    
    /// Actualiza una categoría existente
    pub fn update_category(category_id: i32, dto: UpdateCategoryDto, conn: &mut DbConnection) -> AppResult<CategoryDto> {
        use crate::db::schema::categories::dsl::*;
        
        // Verificar que la categoría existe
        let category = categories
            .filter(id.eq(category_id))
            .first::<Category>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Categoría con ID {} no encontrada", category_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        // Verificar si se está cambiando el slug y si el nuevo slug ya existe
        if let Some(ref new_slug) = dto.slug {
            if new_slug != &category.slug {
                let slug_exists = categories
                    .filter(slug.eq(new_slug))
                    .first::<Category>(conn)
                    .is_ok();
                
                if slug_exists {
                    return Err(AppError::ValidationError(format!("Ya existe una categoría con el slug '{}'", new_slug)));
                }
            }
        }
        
        // Generar slug si se proporciona un nuevo nombre pero no un nuevo slug
        let slug_value = match &dto.name {
            Some(new_name) if dto.slug.is_none() => Some(slugify(new_name)),
            _ => dto.slug.clone()
        };
        
        // Preparar datos para actualización
        let update_data = UpdateCategory {
            name: dto.name,
            slug: slug_value,
            description: dto.description,
            updated_at: Some(Local::now().naive_local()),
        };
        
        // Actualizar la categoría
        let updated_category = diesel::update(categories.filter(id.eq(category_id)))
            .set(&update_data)
            .get_result::<Category>(conn)
            .map_err(|e| AppError::DatabaseError(e))?;
        
        Ok(CategoryDto {
            id: updated_category.id,
            name: updated_category.name,
            slug: updated_category.slug,
            description: updated_category.description,
            created_at: updated_category.created_at,
            updated_at: updated_category.updated_at,
        })
    }
    
    /// Elimina una categoría
    pub fn delete_category(category_id: i32, conn: &mut DbConnection) -> AppResult<()> {
        use crate::db::schema::categories::dsl::*;
        use crate::db::schema::post_categories;
        
        // Verificar que la categoría existe
        let _ = categories
            .filter(id.eq(category_id))
            .first::<Category>(conn)
            .map_err(|e| match e {
                DieselError::NotFound => AppError::NotFound(format!("Categoría con ID {} no encontrada", category_id)),
                _ => AppError::DatabaseError(e)
            })?;
        
        // Iniciar transacción
        conn.transaction(|conn| {
            // Eliminar relaciones con posts
            diesel::delete(post_categories::table.filter(post_categories::category_id.eq(category_id)))
                .execute(conn)
                .map_err(|e| AppError::DatabaseError(e))?;
            
            // Eliminar la categoría
            let deleted = diesel::delete(categories.filter(id.eq(category_id)))
                .execute(conn)
                .map_err(|e| AppError::DatabaseError(e))?;
            
            if deleted == 0 {
                return Err(AppError::NotFound(format!("No se pudo eliminar la categoría con ID {}", category_id)));
            }
            
            Ok(())
        })
    }
} 