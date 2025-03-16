# Guía de Contribución

¡Gracias por tu interés en contribuir al proyecto Blog-Rust! Esta guía te ayudará a entender el proceso de contribución.

## Código de Conducta

Al participar en este proyecto, te comprometes a mantener un ambiente respetuoso y colaborativo. Esperamos que todos los contribuyentes:

- Sean respetuosos y considerados con los demás
- Acepten críticas constructivas
- Se centren en lo que es mejor para la comunidad
- Muestren empatía hacia otros miembros de la comunidad

## ¿Cómo puedo contribuir?

### Reportar Bugs

Si encuentras un bug, por favor crea un issue en GitHub con la siguiente información:

- Un título claro y descriptivo
- Pasos detallados para reproducir el problema
- Comportamiento esperado y comportamiento actual
- Capturas de pantalla si es posible
- Cualquier información adicional relevante

### Sugerir Mejoras

Si tienes ideas para mejorar el proyecto, puedes crear un issue con:

- Un título claro y descriptivo
- Una descripción detallada de la mejora propuesta
- Explicación de por qué esta mejora sería útil
- Posibles implementaciones si las tienes

### Pull Requests

1. Haz fork del repositorio
2. Clona tu fork: `git clone https://github.com/tu-usuario/blog-rust.git`
3. Crea una rama para tu contribución: `git checkout -b feature/nueva-funcionalidad`
4. Realiza tus cambios
5. Asegúrate de que el código pasa todas las pruebas: `cargo test`
6. Haz commit de tus cambios: `git commit -m "Añadir nueva funcionalidad"`
7. Sube tus cambios: `git push origin feature/nueva-funcionalidad`
8. Crea un Pull Request en GitHub

## Estilo de Código

Por favor, sigue estas pautas de estilo:

- Utiliza `rustfmt` para formatear tu código: `cargo fmt`
- Asegúrate de que tu código pasa `clippy`: `cargo clippy`
- Escribe comentarios para código complejo
- Mantén las funciones pequeñas y con una única responsabilidad
- Nombra las variables y funciones de manera descriptiva
- Escribe pruebas para tu código

## Estructura del Proyecto

Antes de contribuir, familiarízate con la estructura del proyecto:

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

## Proceso de Desarrollo

1. **Planificación**: Discute tu idea en un issue antes de comenzar a trabajar
2. **Implementación**: Desarrolla tu funcionalidad en una rama separada
3. **Pruebas**: Asegúrate de que tu código pasa todas las pruebas
4. **Revisión**: Solicita revisión de tu código mediante un Pull Request
5. **Iteración**: Realiza los cambios sugeridos en la revisión
6. **Fusión**: Una vez aprobado, tu código será fusionado en la rama principal

## Convenciones de Commit

Sigue estas convenciones para tus mensajes de commit:

- `feat`: Nueva funcionalidad
- `fix`: Corrección de un bug
- `docs`: Cambios en la documentación
- `style`: Cambios que no afectan al significado del código (espacios, formato, etc.)
- `refactor`: Cambios en el código que no corrigen bugs ni añaden funcionalidades
- `test`: Añadir o corregir pruebas
- `chore`: Cambios en el proceso de build o herramientas auxiliares

Ejemplo: `feat: Añadir sistema de notificaciones por email`

## Pruebas

Antes de enviar un Pull Request, asegúrate de que:

1. Has añadido pruebas para tu código
2. Todas las pruebas pasan: `cargo test`
3. No hay warnings de clippy: `cargo clippy`
4. El código está formateado correctamente: `cargo fmt`

## Documentación

Si añades nuevas funcionalidades, asegúrate de:

1. Actualizar la documentación relevante
2. Añadir comentarios de documentación a las funciones públicas
3. Actualizar el README.md si es necesario

## Preguntas

Si tienes alguna pregunta sobre cómo contribuir, no dudes en crear un issue con tu pregunta.

¡Gracias por contribuir a Blog-Rust!
