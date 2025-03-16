use actix_web::{web, HttpResponse, Responder, post, get};
use crate::config::database::DbPool;
use crate::models::dto::{LoginDto, CreateUserDto};
use crate::services::UserService;
use crate::utils::error::AppError;
use crate::auth::AuthenticatedUser;

#[post("/login")]
pub async fn login(login: web::Json<LoginDto>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match UserService::login(login.into_inner(), &mut conn) {
        Ok(auth_response) => HttpResponse::Ok().json(auth_response),
        Err(AppError::AuthenticationError(msg)) => HttpResponse::Unauthorized().json(msg),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al iniciar sesión: {:?}", e);
            HttpResponse::InternalServerError().json("Error al iniciar sesión")
        }
    }
}

#[post("/register")]
pub async fn register(user: web::Json<CreateUserDto>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    match UserService::create_user(user.into_inner(), &mut conn) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(AppError::ValidationError(msg)) => HttpResponse::BadRequest().json(msg),
        Err(e) => {
            log::error!("Error al registrar usuario: {:?}", e);
            HttpResponse::InternalServerError().json("Error al registrar usuario")
        }
    }
}

#[get("/refresh")]
pub async fn refresh_token(user: AuthenticatedUser, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    if user_id == 0 {
        return HttpResponse::BadRequest().json("ID de usuario inválido");
    }

    match UserService::refresh_token(user_id, &mut conn) {
        Ok(token_response) => HttpResponse::Ok().json(token_response),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al refrescar token: {:?}", e);
            HttpResponse::InternalServerError().json("Error al refrescar token")
        }
    }
}

#[get("/me")]
pub async fn get_current_user(user: AuthenticatedUser, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json("Error de conexión a la base de datos"),
    };

    let user_id = user.0.sub.parse::<i32>().unwrap_or(0);
    if user_id == 0 {
        return HttpResponse::BadRequest().json("ID de usuario inválido");
    }

    match UserService::get_user_by_id(user_id, &mut conn) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(AppError::NotFound(msg)) => HttpResponse::NotFound().json(msg),
        Err(e) => {
            log::error!("Error al obtener usuario actual: {:?}", e);
            HttpResponse::InternalServerError().json("Error al obtener usuario actual")
        }
    }
} 