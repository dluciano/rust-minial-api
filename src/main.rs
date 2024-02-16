use actix_web::{web, App, HttpServer};
use data_types::structs::AppData;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io::Result;

pub mod services;
pub mod data_types;
pub mod middleware;
pub mod routes;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let addrs = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port_str = env::var("PORT").unwrap_or("8080".to_string());
    let port: u16 = port_str.parse().expect("Failed to parse PORT as a u16");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");
    HttpServer::new(move || {
        App::new().service(
            web::scope("/api/v1")
                .app_data(web::Data::new(AppData { pool: pool.clone() }))
                .wrap(middleware::CaptureUri)
                .service(routes::health())
                .service(routes::blog()), // .service(routes::tag()),
        )
    })
    .bind((addrs, port))?
    .run()
    .await
    .expect("\nERROR: src/main.rs: server initialization fail\n");

    Ok(())
}
