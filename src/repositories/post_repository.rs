use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use chrono::Local;
use crate::config::database::DbConnection;
use crate::models::entities::{Post, NewPost, UpdatePost, NewPostCategory};
use crate::models::entities::post_category::PostCategory;
use crate::db::schema::{posts, post_categories};
use crate::utils::slug::slugify;

pub struct PostRepository;

impl PostRepository {
    pub fn find_all(conn: &mut DbConnection) -> QueryResult<Vec<Post>> {
        use crate::db::schema::posts::dsl::*;
        posts.order(id.desc()).load::<Post>(conn)
    }
    
    pub fn find_by_id(post_id: i32, conn: &mut DbConnection) -> QueryResult<Post> {
        use crate::db::schema::posts::dsl::*;
        posts.filter(id.eq(post_id)).first::<Post>(conn)
    }
    
    pub fn find_by_slug(post_slug: &str, conn: &mut DbConnection) -> QueryResult<Post> {
        use crate::db::schema::posts::dsl::*;
        posts.filter(slug.eq(post_slug)).first::<Post>(conn)
    }
    
    pub fn create(new_post: &NewPost, conn: &mut DbConnection) -> QueryResult<Post> {
        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result(conn)
    }
    
    pub fn create_with_categories(
        title: &str, 
        body: &str, 
        slug_str: Option<&str>, 
        category_ids: &[i32], 
        conn: &mut DbConnection
    ) -> QueryResult<Post> {
        // Generar slug si no se proporciona
        let slug_value = match slug_str {
            Some(s) => s.to_string(),
            None => slugify(title),
        };
        
        // Crear el post
        let now = Local::now().naive_local();
        let new_post = NewPost {
            title: title.to_string(),
            body: body.to_string(),
            slug: slug_value,
            created_at: Some(now),
            updated_at: Some(now),
        };
        
        // Iniciar transacción
        conn.transaction(|conn| {
            // Insertar post
            let post = diesel::insert_into(posts::table)
                .values(&new_post)
                .get_result::<Post>(conn)?;
            
            // Insertar relaciones con categorías
            for &category_id in category_ids {
                let new_post_category = NewPostCategory {
                    post_id: post.id,
                    category_id,
                };
                
                diesel::insert_into(post_categories::table)
                    .values(&new_post_category)
                    .execute(conn)?;
            }
            
            Ok(post)
        })
    }
    
    pub fn update(post_id: i32, post_data: &UpdatePost, conn: &mut DbConnection) -> QueryResult<Post> {
        use crate::db::schema::posts::dsl::*;
        
        let mut update_data = post_data.clone();
        update_data.updated_at = Some(Local::now().naive_local());
        
        diesel::update(posts.filter(id.eq(post_id)))
            .set(update_data)
            .get_result::<Post>(conn)
    }
    
    pub fn update_with_categories(
        post_id: i32, 
        title_opt: Option<&str>, 
        body_opt: Option<&str>, 
        slug_opt: Option<&str>, 
        category_ids_opt: Option<&[i32]>, 
        conn: &mut DbConnection
    ) -> QueryResult<Post> {
        // Iniciar transacción
        conn.transaction(|conn| {
            // Preparar datos de actualización
            let mut update_data = UpdatePost {
                title: title_opt.map(|s| s.to_string()),
                body: body_opt.map(|s| s.to_string()),
                slug: slug_opt.map(|s| s.to_string()),
                updated_at: Some(Local::now().naive_local()),
            };
            
            // Si se proporciona título pero no slug, generar slug
            if update_data.title.is_some() && update_data.slug.is_none() {
                let title = update_data.title.as_ref().unwrap();
                update_data.slug = Some(slugify(title));
            }
            
            // Actualizar post
            let updated_post = Self::update(post_id, &update_data, conn)?;
            
            // Si se proporcionan categorías, actualizar relaciones
            if let Some(category_ids) = category_ids_opt {
                // Eliminar relaciones existentes
                diesel::delete(post_categories::table.filter(post_categories::post_id.eq(post_id)))
                    .execute(conn)?;
                
                // Insertar nuevas relaciones
                for &category_id in category_ids {
                    let new_post_category = NewPostCategory {
                        post_id,
                        category_id,
                    };
                    
                    diesel::insert_into(post_categories::table)
                        .values(&new_post_category)
                        .execute(conn)?;
                }
            }
            
            Ok(updated_post)
        })
    }
    
    pub fn delete(post_id: i32, conn: &mut DbConnection) -> QueryResult<usize> {
        use crate::db::schema::posts::dsl::*;
        
        // Iniciar transacción
        conn.transaction(|conn| {
            // Eliminar relaciones con categorías
            diesel::delete(post_categories::table.filter(post_categories::post_id.eq(post_id)))
                .execute(conn)?;
            
            // Eliminar post
            let result = diesel::delete(posts.filter(id.eq(post_id))).execute(conn)?;
            
            Ok(result)
        })
    }
    
    pub fn get_categories(post_id_param: i32, conn: &mut DbConnection) -> QueryResult<Vec<crate::models::entities::Category>> {
        use crate::db::schema::categories::dsl::*;
        use crate::db::schema::post_categories::dsl::{post_categories, post_id};
        
        post_categories
            .filter(post_id.eq(post_id_param))
            .inner_join(categories)
            .select(categories::all_columns())
            .load::<crate::models::entities::Category>(conn)
    }
} 