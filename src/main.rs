use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use sqlx::PgPool;
use std::env;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok(); // Load .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to DB");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
