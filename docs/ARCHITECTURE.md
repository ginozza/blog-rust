# Arquitectura del Proyecto

Este documento describe la arquitectura y los patrones de diseño utilizados en el proyecto Blog-Rust.

## Visión General

Blog-Rust sigue una arquitectura en capas con una clara separación de responsabilidades. La aplicación está diseñada siguiendo principios de Clean Architecture y Domain-Driven Design (DDD).

## Estructura de Directorios

```
src/
├── api/              # Capa de presentación (controladores y rutas)
├── auth/             # Autenticación y autorización
├── config/           # Configuración de la aplicación
├── db/               # Esquema de la base de datos
├── models/           # Modelos de datos
│   ├── dto/          # Data Transfer Objects
│   └── entities/     # Entidades de la base de datos
├── repositories/     # Capa de acceso a datos
├── services/         # Capa de lógica de negocio
└── utils/            # Utilidades y helpers
```

## Capas de la Aplicación

### 1. Capa de Presentación (API)

La capa de presentación se encarga de manejar las solicitudes HTTP, validar los datos de entrada y devolver respuestas apropiadas. Está implementada utilizando Actix Web.

**Componentes principales:**
- **Controladores**: Manejan las solicitudes HTTP y delegan la lógica de negocio a los servicios.
- **Rutas**: Definen los endpoints de la API y los mapean a los controladores correspondientes.
- **Middleware**: Implementa funcionalidades como autenticación, logging, CORS, etc.

### 2. Capa de Servicios

La capa de servicios contiene la lógica de negocio de la aplicación. Actúa como intermediario entre la capa de presentación y la capa de acceso a datos.

**Responsabilidades:**
- Implementar reglas de negocio
- Validar datos
- Coordinar operaciones entre múltiples repositorios
- Manejar transacciones

### 3. Capa de Repositorios

La capa de repositorios proporciona una abstracción sobre el acceso a datos. Implementa el patrón Repository para desacoplar la lógica de negocio de la tecnología de persistencia.

**Responsabilidades:**
- Realizar operaciones CRUD en la base de datos
- Mapear entre entidades y DTOs
- Encapsular consultas complejas

### 4. Capa de Modelos

La capa de modelos define las estructuras de datos utilizadas en la aplicación.

**Componentes:**
- **Entidades**: Representan las tablas de la base de datos.
- **DTOs (Data Transfer Objects)**: Utilizados para transferir datos entre capas y para la serialización/deserialización.

## Patrones de Diseño

### Patrón Repository

El patrón Repository se utiliza para abstraer el acceso a datos. Cada entidad tiene su propio repositorio que encapsula la lógica de acceso a datos.

**Ejemplo:**
```rust
pub struct PostRepository;

impl PostRepository {
    pub fn get_all_posts(conn: &mut DbConnection) -> AppResult<Vec<Post>> {
        // Implementación
    }
    
    pub fn get_post_by_id(post_id: i32, conn: &mut DbConnection) -> AppResult<Post> {
        // Implementación
    }
    
    // Otros métodos
}
```

### Patrón Service

El patrón Service encapsula la lógica de negocio y coordina las operaciones entre múltiples repositorios.

**Ejemplo:**
```rust
pub struct PostService;

impl PostService {
    pub fn get_post_detail(post_id: i32, conn: &mut DbConnection) -> AppResult<PostDetailDto> {
        // Implementación que coordina operaciones entre PostRepository, CategoryRepository y CommentRepository
    }
    
    // Otros métodos
}
```

### Patrón DTO (Data Transfer Object)

Los DTOs se utilizan para transferir datos entre capas y para la serialización/deserialización.

**Ejemplo:**
```rust
#[derive(Serialize, Deserialize)]
pub struct PostDto {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub slug: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
```

### Middleware

Se utiliza middleware para implementar funcionalidades transversales como autenticación, logging, CORS, etc.

**Ejemplo:**
```rust
pub struct JwtAuth {
    pub required_role: Option<String>,
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
{
    // Implementación
}
```

## Flujo de Datos

1. El cliente envía una solicitud HTTP a un endpoint de la API.
2. El middleware procesa la solicitud (autenticación, logging, etc.).
3. El controlador correspondiente recibe la solicitud y extrae los datos necesarios.
4. El controlador llama al servicio apropiado, pasando los datos necesarios.
5. El servicio implementa la lógica de negocio, utilizando uno o más repositorios para acceder a los datos.
6. Los repositorios realizan operaciones en la base de datos y devuelven los resultados al servicio.
7. El servicio procesa los resultados y los devuelve al controlador.
8. El controlador construye una respuesta HTTP apropiada y la devuelve al cliente.

## Manejo de Errores

El proyecto utiliza un enfoque centralizado para el manejo de errores, con un tipo de error personalizado (`AppError`) que puede representar diferentes tipos de errores:

```rust
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    DatabaseError(DieselError),
    AuthenticationError(String),
    AuthorizationError(String),
    InternalServerError(String),
}
```

Los errores se propagan a través de las capas utilizando el tipo `AppResult<T>`, que es un alias para `Result<T, AppError>`.

## Autenticación y Autorización

La autenticación se implementa utilizando JSON Web Tokens (JWT). El flujo es el siguiente:

1. El usuario envía sus credenciales (email y contraseña) al endpoint de login.
2. El servicio de autenticación verifica las credenciales y, si son válidas, genera un token JWT.
3. El token JWT se devuelve al cliente.
4. El cliente incluye el token en el encabezado `Authorization` de las solicitudes posteriores.
5. El middleware de autenticación verifica el token y, si es válido, permite que la solicitud continúe.

La autorización se implementa utilizando roles. El token JWT incluye el rol del usuario, y el middleware de autenticación puede verificar si el usuario tiene el rol requerido para acceder a un recurso específico.

## Conclusión

La arquitectura de Blog-Rust está diseñada para ser modular, mantenible y escalable. La clara separación de responsabilidades facilita la comprensión del código y permite realizar cambios con confianza.
