use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use crate::config::database::DbPool;
use crate::models::dto::{CreateCategoryDto, UpdateCategoryDto};
use crate::services::CategoryService;
use crate::utils::error::AppError;

#[get("")]
pub async fn get_all_categories(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match CategoryService::get_all_categories(&mut conn) {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => {
            log::error!("Error al obtener categorías: {:?}", e);
            HttpResponse::InternalServerError().json("Error al obtener categorías")
        }
    }
}

#[get("/{id}")]
pub async fn get_category_by_id(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let category_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match CategoryService::get_category_by_id(category_id, &mut conn) {
        Ok(category) => HttpResponse::Ok().json(category),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al obtener categoría: {:?}", e);
            HttpResponse::InternalServerError().json("Error al obtener categoría")
        }
    }
}

#[post("")]
pub async fn create_category(category: web::Json<CreateCategoryDto>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match CategoryService::create_category(category.into_inner(), &mut conn) {
        Ok(category) => HttpResponse::Created().json(category),
        Err(AppError::ValidationError(msg)) => HttpResponse::BadRequest().json(msg),
        Err(e) => {
            log::error!("Error al crear categoría: {:?}", e);
            HttpResponse::InternalServerError().json("Error al crear categoría")
        }
    }
}

#[put("/{id}")]
pub async fn update_category(
    path: web::Path<i32>,
    category: web::Json<UpdateCategoryDto>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let category_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match CategoryService::update_category(category_id, category.into_inner(), &mut conn) {
        Ok(category) => HttpResponse::Ok().json(category),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(AppError::ValidationError(msg)) => HttpResponse::BadRequest().json(msg),
        Err(e) => {
            log::error!("Error al actualizar categoría: {:?}", e);
            HttpResponse::InternalServerError().json("Error al actualizar categoría")
        }
    }
}

#[delete("/{id}")]
pub async fn delete_category(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let category_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match CategoryService::delete_category(category_id, &mut conn) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al eliminar categoría: {:?}", e);
            HttpResponse::InternalServerError().json("Error al eliminar categoría")
        }
    }
} 