pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use crate::config::database::DbPool;

/// Inicializa el pool de conexiones a la base de datos
pub fn init_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("No se pudo crear el pool de conexiones")
}

// Eliminamos esta función ya que no es necesaria y causa problemas de tipo
// La conexión se puede obtener directamente con pool.get() 