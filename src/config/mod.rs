pub mod database;

use dotenv::dotenv;
use std::env;

pub fn init() {
    dotenv().ok();
    
    // Inicializar el logger
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}

