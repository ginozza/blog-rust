# Blog-Rust

Una API RESTful para un blog, desarrollada con Rust, Actix Web y Diesel ORM.

## Características

- Autenticación JWT
- CRUD completo para usuarios, categorías, posts y comentarios
- Relaciones entre entidades (posts-categorías, posts-comentarios)
- Validación de datos
- Manejo de errores centralizado
- Arquitectura en capas

## Requisitos

- Rust 1.70 o superior
- PostgreSQL 12 o superior
- Diesel CLI

## Instalación

1. Clonar el repositorio:
   ```bash
   git clone https://github.com/ginozza/blog-rust.git
   cd blog-rust
   ```

2. Instalar Diesel CLI:
   ```bash
   cargo install diesel_cli --no-default-features --features postgres
   ```

3. Configurar la base de datos:
   ```bash
   # Crear archivo .env con la URL de la base de datos
   echo "DATABASE_URL=postgres://usuario:contraseña@localhost/blog_rust" > .env
   
   # Crear la base de datos
   createdb blog_rust
   
   # Ejecutar migraciones
   diesel migration run
   ```

4. Compilar y ejecutar:
   ```bash
   cargo run
   ```

## Uso

La API estará disponible en `http://127.0.0.1:8080/api/`.

### Endpoints principales:

- **Autenticación**: `/api/auth/register`, `/api/auth/login`
- **Usuarios**: `/api/users`
- **Categorías**: `/api/categories`
- **Posts**: `/api/posts`
- **Comentarios**: `/api/comments`

Para más detalles, consulta la [documentación de la API](docs/API.md).

## Estructura del Proyecto

```
src/
├── api/              # Controladores y rutas
├── auth/             # Autenticación y autorización
├── config/           # Configuración de la aplicación
├── db/               # Esquema de la base de datos
├── models/           # Modelos de datos
├── repositories/     # Acceso a datos
├── services/         # Lógica de negocio
└── utils/            # Utilidades
```

Para más detalles, consulta la [documentación de arquitectura](docs/ARCHITECTURE.md).

## Documentación

- [API](docs/API.md): Documentación detallada de todos los endpoints.
- [Arquitectura](docs/ARCHITECTURE.md): Descripción de la arquitectura y patrones de diseño.
- [Base de Datos](docs/DATABASE.md): Estructura y configuración de la base de datos.

## Licencia

Este proyecto está licenciado bajo la [Licencia MIT](LICENSE).
