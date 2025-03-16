pub mod controllers;

use actix_web::{web, Scope};
use crate::api::controllers::{
    user_controller,
    category_controller,
    post_controller,
    comment_controller,
    auth_controller
};
use crate::auth::{JwtAuth, OptionalJwtAuth};

/// Configura todas las rutas de la API
pub fn configure_routes() -> Scope {
    web::scope("/api")
        // Rutas de autenticación
        .service(
            web::scope("/auth")
                // Rutas públicas
                .service(auth_controller::login)
                .service(auth_controller::register)
                // Rutas protegidas
                .service(
                    web::scope("/protected")
                        .wrap(JwtAuth::new()) // Requiere autenticación
                        .service(auth_controller::refresh_token)
                        .service(auth_controller::get_current_user)
                )
        )
        // Rutas protegidas para usuarios
        .service(
            web::scope("/users")
                .wrap(JwtAuth::new()) // Requiere autenticación
                .service(user_controller::get_all_users)
                .service(user_controller::get_user_by_id)
                .service(user_controller::create_user)
                .service(user_controller::update_user)
                .service(user_controller::delete_user)
        )
        .service(
            web::scope("/categories")
                .service(category_controller::get_all_categories) // Público
                .service(category_controller::get_category_by_id) // Público
                .service(
                    web::scope("")
                        .wrap(JwtAuth::new()) // Requiere autenticación
                        .service(category_controller::create_category)
                        .service(category_controller::update_category)
                        .service(category_controller::delete_category)
                )
        )
        .service(
            web::scope("/posts")
                .service(post_controller::get_all_posts) // Público
                .service(post_controller::get_post_by_id) // Público
                .service(post_controller::get_post_detail) // Público
                .service(post_controller::get_post_by_slug) // Público
                .service(
                    web::scope("")
                        .wrap(JwtAuth::new()) // Requiere autenticación
                        .service(post_controller::create_post)
                        .service(post_controller::update_post)
                        .service(post_controller::delete_post)
                )
        )
        .service(
            web::scope("/comments")
                .wrap(OptionalJwtAuth::new()) // Middleware JWT opcional
                .service(comment_controller::get_all_comments) // Público
                .service(comment_controller::get_comment_by_id) // Público
                .service(comment_controller::get_comments_by_post) // Público
                .service(comment_controller::create_comment) // Público - permitir comentarios anónimos
                .service(
                    web::scope("")
                        .wrap(JwtAuth::new()) // Requiere autenticación
                        .service(comment_controller::update_comment)
                        .service(comment_controller::delete_comment)
                )
        )
} 