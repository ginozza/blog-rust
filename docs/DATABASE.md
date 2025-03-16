# Configuración de la Base de Datos

Este documento detalla la configuración y estructura de la base de datos utilizada en el proyecto Blog-Rust.

## Configuración

El proyecto utiliza PostgreSQL como sistema de gestión de base de datos y Diesel como ORM (Object-Relational Mapping).

### Requisitos

- PostgreSQL 12 o superior
- Diesel CLI (`cargo install diesel_cli --no-default-features --features postgres`)

### Configuración Inicial

1. Crear una base de datos en PostgreSQL:
   ```sql
   CREATE DATABASE blog_rust;
   ```

2. Configurar las variables de entorno en el archivo `.env`:
   ```
   DATABASE_URL=postgres://usuario:contraseña@localhost/blog_rust
   ```

3. Ejecutar las migraciones:
   ```bash
   diesel migration run
   ```

## Estructura de la Base de Datos

### Tablas

#### users

Almacena la información de los usuarios registrados.

| Columna      | Tipo          | Descripción                                |
|--------------|---------------|--------------------------------------------|
| id           | SERIAL        | Identificador único (clave primaria)       |
| username     | VARCHAR       | Nombre de usuario (único)                  |
| email        | VARCHAR       | Correo electrónico (único)                 |
| password_hash| VARCHAR       | Hash de la contraseña                      |
| role         | VARCHAR       | Rol del usuario (admin, user, etc.)        |
| created_at   | TIMESTAMP     | Fecha y hora de creación                   |
| updated_at   | TIMESTAMP     | Fecha y hora de última actualización       |

#### categories

Almacena las categorías para clasificar los posts.

| Columna      | Tipo          | Descripción                                |
|--------------|---------------|--------------------------------------------|
| id           | SERIAL        | Identificador único (clave primaria)       |
| name         | VARCHAR       | Nombre de la categoría                     |
| slug         | VARCHAR       | Slug para URLs amigables (único)           |
| description  | TEXT          | Descripción de la categoría (opcional)     |
| created_at   | TIMESTAMP     | Fecha y hora de creación                   |
| updated_at   | TIMESTAMP     | Fecha y hora de última actualización       |

#### posts

Almacena los artículos del blog.

| Columna      | Tipo          | Descripción                                |
|--------------|---------------|--------------------------------------------|
| id           | SERIAL        | Identificador único (clave primaria)       |
| title        | VARCHAR       | Título del post                            |
| body         | TEXT          | Contenido del post                         |
| slug         | VARCHAR       | Slug para URLs amigables (único)           |
| created_at   | TIMESTAMP     | Fecha y hora de creación                   |
| updated_at   | TIMESTAMP     | Fecha y hora de última actualización       |

#### post_categories

Tabla de relación muchos a muchos entre posts y categorías.

| Columna      | Tipo          | Descripción                                |
|--------------|---------------|--------------------------------------------|
| id           | SERIAL        | Identificador único (clave primaria)       |
| post_id      | INTEGER       | ID del post (clave foránea)                |
| category_id  | INTEGER       | ID de la categoría (clave foránea)         |

#### comments

Almacena los comentarios de los posts.

| Columna      | Tipo          | Descripción                                |
|--------------|---------------|--------------------------------------------|
| id           | SERIAL        | Identificador único (clave primaria)       |
| post_id      | INTEGER       | ID del post (clave foránea)                |
| user_id      | INTEGER       | ID del usuario (clave foránea, opcional)   |
| author_name  | VARCHAR       | Nombre del autor (para comentarios anónimos)|
| author_email | VARCHAR       | Email del autor (opcional)                 |
| content      | TEXT          | Contenido del comentario                   |
| created_at   | TIMESTAMP     | Fecha y hora de creación                   |
| updated_at   | TIMESTAMP     | Fecha y hora de última actualización       |

### Relaciones

- Un **usuario** puede crear múltiples **posts** y **comentarios**.
- Un **post** puede pertenecer a múltiples **categorías** (relación muchos a muchos).
- Un **post** puede tener múltiples **comentarios**.
- Un **comentario** pertenece a un único **post**.
- Un **comentario** puede estar asociado a un **usuario** o ser anónimo.

### Índices

- `users`: índices en `id`, `username` y `email`.
- `categories`: índices en `id` y `slug`.
- `posts`: índices en `id` y `slug`.
- `post_categories`: índices en `post_id` y `category_id`.
- `comments`: índices en `id` y `post_id`.

## Migraciones

Las migraciones se encuentran en el directorio `migrations/` y se ejecutan en orden cronológico. Cada migración tiene dos archivos:

- `up.sql`: Contiene las instrucciones para aplicar la migración.
- `down.sql`: Contiene las instrucciones para revertir la migración.

### Crear una Nueva Migración

```bash
diesel migration generate nombre_de_la_migracion
```

### Ejecutar Migraciones

```bash
diesel migration run
```

### Revertir la Última Migración

```bash
diesel migration revert
```

### Regenerar el Esquema

```bash
diesel print-schema > src/db/schema/mod.rs
```

## Modelo de Datos

El proyecto utiliza el patrón Repository para acceder a los datos. Los modelos se dividen en:

- **Entidades**: Representan las tablas de la base de datos.
- **DTOs (Data Transfer Objects)**: Utilizados para transferir datos entre capas.

### Ejemplo de Entidad

```rust
#[derive(Queryable, Identifiable, Associations, Debug)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub slug: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
```

### Ejemplo de DTO

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
