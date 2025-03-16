pub mod jwt;
pub mod middleware;
pub mod password;

pub use jwt::{Claims, create_token};
pub use middleware::{JwtAuth, AuthenticatedUser, OptionalAuthenticatedUser, OptionalJwtAuth};
pub use password::{hash_password, verify_password}; 