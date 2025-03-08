use actix_web::{web, HttpResponse, Responder};
use crate::models::user::User;
use sqlx::PgPool;
use bcrypt::{verify};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, TokenData};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn signup(pool: web::Data<PgPool>, item: web::Json<SignupRequest>) -> impl Responder {
    let password_hash = User::hash_password(&item.password).unwrap();

    let result = sqlx::query!(
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
        item.name, item.email, password_hash
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().body("User created successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Error signing up"),
    }
}

pub async fn login(pool: web::Data<PgPool>, item: web::Json<LoginRequest>) -> impl Responder {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&item.email)
        .fetch_one(pool.get_ref())
        .await;

    match user {
        Ok(user) => {
            if user.verify_password(&item.password).unwrap_or(false) {
                let claims = Claims { sub: user.id.to_string(), exp: 10000000000 };
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
                HttpResponse::Ok().json(token)
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        },
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

pub async fn current_user(token: web::Header<String>) -> impl Responder {
    match decode::<Claims>(&token.0, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()) {
        Ok(token_data) => HttpResponse::Ok().json(token_data.claims),
        Err(_) => HttpResponse::Unauthorized().body("Not authenticated"),
    }
}
