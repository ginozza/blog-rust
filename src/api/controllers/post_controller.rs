use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use crate::config::database::DbPool;
use crate::models::dto::{CreatePostDto, UpdatePostDto};
use crate::services::PostService;
use crate::utils::error::AppError;

#[get("")]
pub async fn get_all_posts(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match PostService::get_all_posts(&mut conn) {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            log::error!("Error al obtener posts: {:?}", e);
            HttpResponse::InternalServerError().json("Error al obtener posts")
        }
    }
}

#[get("/{id}")]
pub async fn get_post_by_id(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let post_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match PostService::get_post_by_id(post_id, &mut conn) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al obtener post: {:?}", e);
            HttpResponse::InternalServerError().json("Error al obtener post")
        }
    }
}

#[get("/{id}/detail")]
pub async fn get_post_detail(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let post_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match PostService::get_post_detail(post_id, &mut conn) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al obtener detalle del post: {:?}", e);
            HttpResponse::InternalServerError().json("Error al obtener detalle del post")
        }
    }
}

#[get("/slug/{slug}")]
pub async fn get_post_by_slug(path: web::Path<String>, pool: web::Data<DbPool>) -> impl Responder {
    let slug = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match PostService::get_post_by_slug(&slug, &mut conn) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al obtener post por slug: {:?}", e);
            HttpResponse::InternalServerError().json("Error al obtener post por slug")
        }
    }
}

#[post("")]
pub async fn create_post(post: web::Json<CreatePostDto>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match PostService::create_post(post.into_inner(), &mut conn) {
        Ok(post) => HttpResponse::Created().json(post),
        Err(AppError::ValidationError(msg)) => HttpResponse::BadRequest().json(msg),
        Err(e) => {
            log::error!("Error al crear post: {:?}", e);
            HttpResponse::InternalServerError().json("Error al crear post")
        }
    }
}

#[put("/{id}")]
pub async fn update_post(
    path: web::Path<i32>,
    post: web::Json<UpdatePostDto>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let post_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match PostService::update_post(post_id, post.into_inner(), &mut conn) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(AppError::ValidationError(msg)) => HttpResponse::BadRequest().json(msg),
        Err(e) => {
            log::error!("Error al actualizar post: {:?}", e);
            HttpResponse::InternalServerError().json("Error al actualizar post")
        }
    }
}

#[delete("/{id}")]
pub async fn delete_post(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let post_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match PostService::delete_post(post_id, &mut conn) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al eliminar post: {:?}", e);
            HttpResponse::InternalServerError().json("Error al eliminar post")
        }
    }
} 