use actix_web::http::StatusCode;
use actix_web::{get, HttpResponse};

#[get("/health")]
async fn health() -> HttpResponse {
    let response = HttpResponse::Ok()
        .status(StatusCode::OK)
        .content_type("application/json")
        .body("{\"health\": \"healthy\"}");
    return response;
}
