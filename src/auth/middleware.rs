use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header::{self},
    Error as ActixError,
    HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::rc::Rc;

use crate::auth::jwt::{validate_token, Claims};

pub struct JwtAuth {
    pub required_role: Option<String>,
}

impl JwtAuth {
    pub fn new() -> Self {
        JwtAuth { required_role: None }
    }

    pub fn role(mut self, role: &str) -> Self {
        self.required_role = Some(role.to_string());
        self
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service: Rc::new(service),
            required_role: self.required_role.clone(),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
    required_role: Option<String>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let required_role = self.required_role.clone();

        Box::pin(async move {
            // Extraer el token del encabezado Authorization
            let auth_header = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok());

            let token = match auth_header {
                Some(auth) if auth.starts_with("Bearer ") => {
                    let token = auth.trim_start_matches("Bearer ").trim();
                    token
                }
                _ => {
                    return Err(ErrorUnauthorized("Token no proporcionado"));
                }
            };

            // Validar el token
            let token_data = validate_token(token)?;
            let claims = token_data.claims;

            // Verificar el rol si es necesario
            if let Some(role) = required_role {
                if claims.role != role {
                    return Err(ErrorUnauthorized("Rol insuficiente"));
                }
            }

            // Añadir los claims al request para que estén disponibles en los handlers
            req.extensions_mut().insert(claims);

            // Continuar con la cadena de middleware
            service.call(req).await
        })
    }
}

// Extractor para obtener los claims del request
#[derive(Clone)]
pub struct AuthenticatedUser(pub Claims);

impl FromRequest for AuthenticatedUser {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let claims = req.extensions().get::<Claims>().cloned();
        match claims {
            Some(claims) => ready(Ok(AuthenticatedUser(claims))),
            None => ready(Err(ErrorUnauthorized("Usuario no autenticado"))),
        }
    }
}

// Extractor opcional para obtener los claims del request sin fallar cuando no hay token
#[derive(Clone)]
pub struct OptionalAuthenticatedUser(pub Option<Claims>);

impl FromRequest for OptionalAuthenticatedUser {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let claims = req.extensions().get::<Claims>().cloned();
        ready(Ok(OptionalAuthenticatedUser(claims)))
    }
}

// Importaciones adicionales para el extractor
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;

// Middleware para autenticación JWT opcional
pub struct OptionalJwtAuth;

impl OptionalJwtAuth {
    pub fn new() -> Self {
        OptionalJwtAuth
    }
}

impl<S, B> Transform<S, ServiceRequest> for OptionalJwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Transform = OptionalJwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(OptionalJwtAuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct OptionalJwtAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for OptionalJwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Extraer el token del encabezado Authorization
            let auth_header = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok());

            // Si hay un token, intentar validarlo
            if let Some(auth) = auth_header {
                if auth.starts_with("Bearer ") {
                    let token = auth.trim_start_matches("Bearer ").trim();
                    
                    // Intentar validar el token
                    if let Ok(token_data) = validate_token(token) {
                        // Añadir los claims al request para que estén disponibles en los handlers
                        req.extensions_mut().insert(token_data.claims);
                    }
                }
            }

            // Continuar con la cadena de middleware, independientemente de si hay token o no
            service.call(req).await
        })
    }
} 