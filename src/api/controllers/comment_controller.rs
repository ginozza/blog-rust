use actix_web::{web, HttpResponse, Responder, get, post, put, delete};
use crate::config::database::DbPool;
use crate::models::dto::{CreateCommentDto, UpdateCommentDto};
use crate::services::CommentService;
use crate::utils::error::AppError;
use crate::auth::{AuthenticatedUser, OptionalAuthenticatedUser};

#[get("")]
pub async fn get_all_comments(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match CommentService::get_all_comments(&mut conn) {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(e) => {
            log::error!("Error al obtener comentarios: {:?}", e);
            HttpResponse::InternalServerError().json("Error al obtener comentarios")
        }
    }
}

#[get("/{id}")]
pub async fn get_comment_by_id(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let comment_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match CommentService::get_comment_by_id(comment_id, &mut conn) {
        Ok(comment) => HttpResponse::Ok().json(comment),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al obtener comentario: {:?}", e);
            HttpResponse::InternalServerError().json("Error al obtener comentario")
        }
    }
}

#[get("/post/{post_id}")]
pub async fn get_comments_by_post(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let post_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match CommentService::get_comments_by_post_id(post_id, &mut conn) {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(e) => {
            log::error!("Error al obtener comentarios del post: {:?}", e);
            HttpResponse::InternalServerError().json("Error al obtener comentarios del post")
        }
    }
}

#[post("")]
pub async fn create_comment(
    comment: web::Json<CreateCommentDto>, 
    auth_user: OptionalAuthenticatedUser, 
    pool: web::Data<DbPool>
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    // Extraer el ID de usuario del token JWT si está disponible
    let mut comment_data = comment.into_inner();
    
    // Si hay un usuario autenticado, usar su ID
    if let Some(claims) = auth_user.0 {
        let user_id = claims.sub.parse::<i32>().unwrap_or(0);
        if user_id > 0 {
            comment_data.user_id = Some(user_id);
        }
    }

    // Si no hay user_id, asegurarse de que hay un author_name
    if comment_data.user_id.is_none() && comment_data.author_name.is_none() {
        comment_data.author_name = Some("Anónimo".to_string());
    }

    match CommentService::create_comment(comment_data, &mut conn) {
        Ok(comment) => HttpResponse::Created().json(comment),
        Err(AppError::ValidationError(msg)) => HttpResponse::BadRequest().json(msg),
        Err(e) => {
            log::error!("Error al crear comentario: {:?}", e);
            HttpResponse::InternalServerError().json("Error al crear comentario")
        }
    }
}

#[put("/{id}")]
pub async fn update_comment(
    path: web::Path<i32>,
    comment: web::Json<UpdateCommentDto>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let comment_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match CommentService::update_comment(comment_id, comment.into_inner(), &mut conn) {
        Ok(comment) => HttpResponse::Ok().json(comment),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(AppError::ValidationError(msg)) => HttpResponse::BadRequest().json(msg),
        Err(e) => {
            log::error!("Error al actualizar comentario: {:?}", e);
            HttpResponse::InternalServerError().json("Error al actualizar comentario")
        }
    }
}

#[delete("/{id}")]
pub async fn delete_comment(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let comment_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match CommentService::delete_comment(comment_id, &mut conn) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al eliminar comentario: {:?}", e);
            HttpResponse::InternalServerError().json("Error al eliminar comentario")
        }
    }
} 