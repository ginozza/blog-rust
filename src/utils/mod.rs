pub mod error;
pub mod validation;
pub mod slug;

// Exportamos públicamente para que sean accesibles desde otros módulos
pub use slug::slugify;
pub use error::{AppError, AppResult}; 