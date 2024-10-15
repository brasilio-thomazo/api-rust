use actix_web::{web, App, HttpServer};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, PgPool};

mod controllers;
mod models;
mod routes;

#[derive(Debug, Serialize)]
struct ResponseError {
    message: String,
}

impl ResponseError {
    fn new(message: String) -> ResponseError {
        ResponseError { message }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let pool = connect().await.expect("Failed to connect to the database");
    server(&pool).await
}

async fn connect() -> Result<PgPool, sqlx::Error> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new().max_connections(5).connect(&url).await
}

async fn server(pool: &PgPool) -> std::io::Result<()> {
    let app_data = web::Data::new(pool.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/ping", web::get().to(|| async { "pong" }))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
