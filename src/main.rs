use actix_web::{App, HttpServer};
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;

mod config;
mod models;
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to database");

    println!("🚀 Server running on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
