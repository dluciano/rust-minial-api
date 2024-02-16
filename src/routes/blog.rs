use crate::data_types::structs::{AppData, Blog};
use crate::services::blogs::commands::update_blog;
use crate::services::blogs::queries::get_all_featured_blogs;
use crate::services::blogs::{delete_blog, get_all_blogs, get_blog_by_id, insert_blog};
use crate::utils::handle_sql_error;
use actix_web::http::StatusCode;
use actix_web::web::{self, Json, Path};
use actix_web::{delete, get, post, put, HttpResponse};

#[post("/blog")]
async fn create_blog_route(blog: Json<Blog>, app_data: web::Data<AppData>) -> HttpResponse {
    let result = insert_blog(&blog.0, &app_data.pool).await;

    match result {
        Ok(entity) => HttpResponse::Created()
            .status(StatusCode::CREATED)
            .content_type("application/json")
            .body(
                serde_json::to_string(&Json(entity))
                    .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
            ),
        Err(e) => handle_sql_error(e),
    }
}

#[get("/blog/featured")]
async fn get_featured_blogs_route(app_data: web::Data<AppData>) -> HttpResponse {
    let result = get_all_featured_blogs(&app_data.pool).await;

    match result {
        Ok(entity) => HttpResponse::Ok().content_type("application/json").body(
            serde_json::to_string(&Json(entity))
                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
        ),
        Err(e) => handle_sql_error(e),
    }
}

#[get("/blogs")]
async fn get_all_blogs_route(app_data: web::Data<AppData>) -> HttpResponse {
    let result = get_all_blogs(&app_data.pool).await;
    match result {
        Ok(entity) => HttpResponse::Ok().content_type("application/json").body(
            serde_json::to_string(&Json(entity))
                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
        ),
        Err(e) => handle_sql_error(e),
    }
}

#[get("/blog/{id}")]
async fn get_blog_by_id_route(id: Path<i32>, app_data: web::Data<AppData>) -> HttpResponse {
    let result = get_blog_by_id(&id.into_inner(), &app_data.pool).await;
    match result {
        Ok(entity) => HttpResponse::Ok().content_type("application/json").body(
            serde_json::to_string(&Json(entity))
                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
        ),
        Err(e) => handle_sql_error(e),
    }
}

#[put("/blog/{id}")]
async fn update_blog_route(
    id: Path<i32>,
    blog: Json<Blog>,
    app_data: web::Data<AppData>,
) -> HttpResponse {
    let result = update_blog(&id, &blog.0, &app_data.pool).await;

    match result {
        Ok(entity) => HttpResponse::Ok().content_type("application/json").body(
            serde_json::to_string(&Json(entity))
                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
        ),
        Err(e) => handle_sql_error(e),
    }
}

#[delete("/blog/{id}")]
async fn delete_blog_route(id: Path<i32>, app_data: web::Data<AppData>) -> HttpResponse {
    let result = delete_blog(&id, &app_data.pool).await;

    match result {
        Ok(_) => HttpResponse::Created()
            .status(StatusCode::NO_CONTENT)
            .content_type("application/json")
            .finish(),
        Err(e) => handle_sql_error(e),
    }
}
