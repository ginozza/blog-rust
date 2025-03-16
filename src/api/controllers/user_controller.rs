use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use crate::config::database::DbPool;
use crate::models::dto::{CreateUserDto, UpdateUserDto};
use crate::services::UserService;
use crate::utils::error::AppError;

#[get("")]
pub async fn get_all_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match UserService::get_all_users(&mut conn) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            log::error!("Error al obtener usuarios: {:?}", e);
            HttpResponse::InternalServerError().json("Error al obtener usuarios")
        }
    }
}

#[get("/{id}")]
pub async fn get_user_by_id(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let user_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match UserService::get_user_by_id(user_id, &mut conn) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al obtener usuario: {:?}", e);
            HttpResponse::InternalServerError().json("Error al obtener usuario")
        }
    }
}

#[post("")]
pub async fn create_user(user: web::Json<CreateUserDto>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match UserService::create_user(user.into_inner(), &mut conn) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(AppError::ValidationError(msg)) => HttpResponse::BadRequest().json(msg),
        Err(e) => {
            log::error!("Error al crear usuario: {:?}", e);
            HttpResponse::InternalServerError().json("Error al crear usuario")
        }
    }
}

#[put("/{id}")]
pub async fn update_user(
    path: web::Path<i32>,
    user: web::Json<UpdateUserDto>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let user_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match UserService::update_user(user_id, user.into_inner(), &mut conn) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(AppError::ValidationError(msg)) => HttpResponse::BadRequest().json(msg),
        Err(e) => {
            log::error!("Error al actualizar usuario: {:?}", e);
            HttpResponse::InternalServerError().json("Error al actualizar usuario")
        }
    }
}

#[delete("/{id}")]
pub async fn delete_user(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let user_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match UserService::delete_user(user_id, &mut conn) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al eliminar usuario: {:?}", e);
            HttpResponse::InternalServerError().json("Error al eliminar usuario")
        }
    }
} 