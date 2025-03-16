# Documentación de la API

Esta documentación detalla todos los endpoints disponibles en la API de Blog-Rust.

## Autenticación

### Registro de usuario

**Endpoint:** `POST /api/auth/register`

**Descripción:** Registra un nuevo usuario en el sistema.

**Cuerpo de la solicitud:**
```json
{
  "username": "string",
  "email": "string",
  "password": "string",
  "role": "string" // Opcional, por defecto "user"
}
```

**Respuesta exitosa (201 Created):**
```json
{
  "id": "integer",
  "username": "string",
  "email": "string",
  "created_at": "datetime",
  "updated_at": "datetime",
  "role": "string"
}
```

### Inicio de sesión

**Endpoint:** `POST /api/auth/login`

**Descripción:** Autentica a un usuario y devuelve un token JWT.

**Cuerpo de la solicitud:**
```json
{
  "email": "string",
  "password": "string"
}
```

**Respuesta exitosa (200 OK):**
```json
{
  "token": "string",
  "user": {
    "id": "integer",
    "username": "string",
    "email": "string",
    "created_at": "datetime",
    "updated_at": "datetime",
    "role": "string"
  }
}
```

### Refrescar token

**Endpoint:** `POST /api/auth/protected/refresh`

**Descripción:** Genera un nuevo token JWT para el usuario autenticado.

**Encabezados:**
- `Authorization: Bearer {token}`

**Respuesta exitosa (200 OK):**
```json
{
  "token": "string"
}
```

### Obtener usuario actual

**Endpoint:** `GET /api/auth/protected/me`

**Descripción:** Devuelve la información del usuario autenticado.

**Encabezados:**
- `Authorization: Bearer {token}`

**Respuesta exitosa (200 OK):**
```json
{
  "id": "integer",
  "username": "string",
  "email": "string",
  "created_at": "datetime",
  "updated_at": "datetime",
  "role": "string"
}
```

## Usuarios

### Obtener todos los usuarios

**Endpoint:** `GET /api/users`

**Descripción:** Devuelve una lista de todos los usuarios.

**Encabezados:**
- `Authorization: Bearer {token}`

**Respuesta exitosa (200 OK):**
```json
[
  {
    "id": "integer",
    "username": "string",
    "email": "string",
    "created_at": "datetime",
    "updated_at": "datetime",
    "role": "string"
  }
]
```

### Obtener usuario por ID

**Endpoint:** `GET /api/users/{id}`

**Descripción:** Devuelve la información de un usuario específico.

**Encabezados:**
- `Authorization: Bearer {token}`

**Respuesta exitosa (200 OK):**
```json
{
  "id": "integer",
  "username": "string",
  "email": "string",
  "created_at": "datetime",
  "updated_at": "datetime",
  "role": "string"
}
```

### Crear usuario

**Endpoint:** `POST /api/users`

**Descripción:** Crea un nuevo usuario.

**Encabezados:**
- `Authorization: Bearer {token}`

**Cuerpo de la solicitud:**
```json
{
  "username": "string",
  "email": "string",
  "password": "string",
  "role": "string" // Opcional, por defecto "user"
}
```

**Respuesta exitosa (201 Created):**
```json
{
  "id": "integer",
  "username": "string",
  "email": "string",
  "created_at": "datetime",
  "updated_at": "datetime",
  "role": "string"
}
```

### Actualizar usuario

**Endpoint:** `PUT /api/users/{id}`

**Descripción:** Actualiza la información de un usuario existente.

**Encabezados:**
- `Authorization: Bearer {token}`

**Cuerpo de la solicitud:**
```json
{
  "username": "string", // Opcional
  "email": "string", // Opcional
  "password": "string", // Opcional
  "role": "string" // Opcional
}
```

**Respuesta exitosa (200 OK):**
```json
{
  "id": "integer",
  "username": "string",
  "email": "string",
  "created_at": "datetime",
  "updated_at": "datetime",
  "role": "string"
}
```

### Eliminar usuario

**Endpoint:** `DELETE /api/users/{id}`

**Descripción:** Elimina un usuario existente.

**Encabezados:**
- `Authorization: Bearer {token}`

**Respuesta exitosa (204 No Content)**

## Categorías

### Obtener todas las categorías

**Endpoint:** `GET /api/categories`

**Descripción:** Devuelve una lista de todas las categorías.

**Respuesta exitosa (200 OK):**
```json
[
  {
    "id": "integer",
    "name": "string",
    "slug": "string",
    "description": "string",
    "created_at": "datetime",
    "updated_at": "datetime"
  }
]
```

### Obtener categoría por ID

**Endpoint:** `GET /api/categories/{id}`

**Descripción:** Devuelve la información de una categoría específica.

**Respuesta exitosa (200 OK):**
```json
{
  "id": "integer",
  "name": "string",
  "slug": "string",
  "description": "string",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Crear categoría

**Endpoint:** `POST /api/categories`

**Descripción:** Crea una nueva categoría.

**Encabezados:**
- `Authorization: Bearer {token}`

**Cuerpo de la solicitud:**
```json
{
  "name": "string",
  "slug": "string", // Opcional, se genera automáticamente si no se proporciona
  "description": "string" // Opcional
}
```

**Respuesta exitosa (201 Created):**
```json
{
  "id": "integer",
  "name": "string",
  "slug": "string",
  "description": "string",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Actualizar categoría

**Endpoint:** `PUT /api/categories/{id}`

**Descripción:** Actualiza la información de una categoría existente.

**Encabezados:**
- `Authorization: Bearer {token}`

**Cuerpo de la solicitud:**
```json
{
  "name": "string", // Opcional
  "slug": "string", // Opcional
  "description": "string" // Opcional
}
```

**Respuesta exitosa (200 OK):**
```json
{
  "id": "integer",
  "name": "string",
  "slug": "string",
  "description": "string",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Eliminar categoría

**Endpoint:** `DELETE /api/categories/{id}`

**Descripción:** Elimina una categoría existente.

**Encabezados:**
- `Authorization: Bearer {token}`

**Respuesta exitosa (204 No Content)**

## Posts

### Obtener todos los posts

**Endpoint:** `GET /api/posts`

**Descripción:** Devuelve una lista de todos los posts.

**Respuesta exitosa (200 OK):**
```json
[
  {
    "id": "integer",
    "title": "string",
    "body": "string",
    "slug": "string",
    "created_at": "datetime",
    "updated_at": "datetime"
  }
]
```

### Obtener post por ID

**Endpoint:** `GET /api/posts/{id}`

**Descripción:** Devuelve la información de un post específico.

**Respuesta exitosa (200 OK):**
```json
{
  "id": "integer",
  "title": "string",
  "body": "string",
  "slug": "string",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Obtener detalles de un post

**Endpoint:** `GET /api/posts/{id}/detail`

**Descripción:** Devuelve la información detallada de un post, incluyendo sus categorías y comentarios.

**Respuesta exitosa (200 OK):**
```json
{
  "id": "integer",
  "title": "string",
  "body": "string",
  "slug": "string",
  "created_at": "datetime",
  "updated_at": "datetime",
  "categories": [
    {
      "id": "integer",
      "name": "string",
      "slug": "string",
      "description": "string"
    }
  ],
  "comments": [
    {
      "id": "integer",
      "post_id": "integer",
      "user_id": "integer",
      "author_name": "string",
      "author_email": "string",
      "content": "string",
      "created_at": "datetime",
      "updated_at": "datetime"
    }
  ]
}
```

### Obtener post por slug

**Endpoint:** `GET /api/posts/slug/{slug}`

**Descripción:** Devuelve la información de un post específico por su slug.

**Respuesta exitosa (200 OK):**
```json
{
  "id": "integer",
  "title": "string",
  "body": "string",
  "slug": "string",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Crear post

**Endpoint:** `POST /api/posts`

**Descripción:** Crea un nuevo post.

**Encabezados:**
- `Authorization: Bearer {token}`

**Cuerpo de la solicitud:**
```json
{
  "title": "string",
  "body": "string",
  "slug": "string", // Opcional, se genera automáticamente si no se proporciona
  "category_ids": ["integer"] // Opcional
}
```

**Respuesta exitosa (201 Created):**
```json
{
  "id": "integer",
  "title": "string",
  "body": "string",
  "slug": "string",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Actualizar post

**Endpoint:** `PUT /api/posts/{id}`

**Descripción:** Actualiza la información de un post existente.

**Encabezados:**
- `Authorization: Bearer {token}`

**Cuerpo de la solicitud:**
```json
{
  "title": "string", // Opcional
  "body": "string", // Opcional
  "slug": "string", // Opcional
  "category_ids": ["integer"] // Opcional
}
```

**Respuesta exitosa (200 OK):**
```json
{
  "id": "integer",
  "title": "string",
  "body": "string",
  "slug": "string",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Eliminar post

**Endpoint:** `DELETE /api/posts/{id}`

**Descripción:** Elimina un post existente.

**Encabezados:**
- `Authorization: Bearer {token}`

**Respuesta exitosa (204 No Content)**

## Comentarios

### Obtener todos los comentarios

**Endpoint:** `GET /api/comments`

**Descripción:** Devuelve una lista de todos los comentarios.

**Respuesta exitosa (200 OK):**
```json
[
  {
    "id": "integer",
    "post_id": "integer",
    "user_id": "integer",
    "author_name": "string",
    "author_email": "string",
    "content": "string",
    "created_at": "datetime",
    "updated_at": "datetime"
  }
]
```

### Obtener comentario por ID

**Endpoint:** `GET /api/comments/{id}`

**Descripción:** Devuelve la información de un comentario específico.

**Respuesta exitosa (200 OK):**
```json
{
  "id": "integer",
  "post_id": "integer",
  "user_id": "integer",
  "author_name": "string",
  "author_email": "string",
  "content": "string",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Obtener comentarios de un post

**Endpoint:** `GET /api/comments/post/{post_id}`

**Descripción:** Devuelve una lista de todos los comentarios de un post específico.

**Respuesta exitosa (200 OK):**
```json
[
  {
    "id": "integer",
    "post_id": "integer",
    "user_id": "integer",
    "author_name": "string",
    "author_email": "string",
    "content": "string",
    "created_at": "datetime",
    "updated_at": "datetime"
  }
]
```

### Crear comentario

**Endpoint:** `POST /api/comments`

**Descripción:** Crea un nuevo comentario. Puede ser anónimo o autenticado.

**Encabezados (opcional):**
- `Authorization: Bearer {token}`

**Cuerpo de la solicitud (anónimo):**
```json
{
  "post_id": "integer",
  "author_name": "string",
  "author_email": "string", // Opcional
  "content": "string"
}
```

**Cuerpo de la solicitud (autenticado):**
```json
{
  "post_id": "integer",
  "content": "string"
}
```

**Respuesta exitosa (201 Created):**
```json
{
  "id": "integer",
  "post_id": "integer",
  "user_id": "integer",
  "author_name": "string",
  "author_email": "string",
  "content": "string",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Actualizar comentario

**Endpoint:** `PUT /api/comments/{id}`

**Descripción:** Actualiza la información de un comentario existente.

**Encabezados:**
- `Authorization: Bearer {token}`

**Cuerpo de la solicitud:**
```json
{
  "content": "string"
}
```

**Respuesta exitosa (200 OK):**
```json
{
  "id": "integer",
  "post_id": "integer",
  "user_id": "integer",
  "author_name": "string",
  "author_email": "string",
  "content": "string",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Eliminar comentario

**Endpoint:** `DELETE /api/comments/{id}`

**Descripción:** Elimina un comentario existente.

**Encabezados:**
- `Authorization: Bearer {token}`

**Respuesta exitosa (204 No Content)**
