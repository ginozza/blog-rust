mod config;
mod db;
mod models;
mod repositories;
mod services;
mod utils;
mod api;
mod auth;

use log::info;
use std::env;
use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use crate::api::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar configuración
    config::init();
    
    // Obtener URL de la base de datos
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada");
    
    // Crear pool de conexiones
    let pool = db::init_pool(&database_url);
    
    info!("Iniciando servidor en http://127.0.0.1:8080");
    
    // Iniciar servidor HTTP
    HttpServer::new(move || {
        // Configurar CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
            
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(configure_routes())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

