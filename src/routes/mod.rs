use actix_web::web;
use crate::handlers::user_handler;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/signup", web::post().to(user_handler::signup))
            .route("/login", web::post().to(user_handler::login))
            .route("/me", web::get().to(user_handler::current_user))
    );
}
