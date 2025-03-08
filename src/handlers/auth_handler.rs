use actix_web::{web, HttpResponse, Responder, HttpRequest};
use sqlx::PgPool;
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, TokenData};
use crate::models::user::User;
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};

const JWT_SECRET: &str = "your-secret-key";  // Change to something secret

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
    sub: String,  // user ID
    exp: usize,   // expiration time
}

// Signup a new user
pub async fn signup(pool: web::Data<PgPool>, item: web::Json<SignupRequest>) -> impl Responder {
    let password_hash = User::hash_password(&item.password).unwrap();

    let query = sqlx::query!(
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id, name, email, password_hash",
        item.name, item.email, password_hash
    )
    .fetch_one(pool.get_ref())
    .await;

    match query {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Error signing up"),
    }
}

// Login a user
pub async fn login(pool: web::Data<PgPool>, item: web::Json<LoginRequest>, id: Identity) -> impl Responder {
    let query = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&item.email)
    .fetch_one(pool.get_ref())
    .await;

    match query {
        Ok(user) => {
            if user.verify_password(&item.password).unwrap_or(false) {
                let claims = Claims {
                    sub: user.id.to_string(),
                    exp: 10000000000, // Set an appropriate expiration time for the token
                };
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_ref()))
                    .unwrap();

                id.remember(token);  // Store the token in the session
                HttpResponse::Ok().json("Login successful")
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        },
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}

// Authenticated user info
pub async fn current_user(id: Identity) -> impl Responder {
    if let Some(token) = id.identity() {
        let token_data: TokenData<Claims> = decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &Validation::default())
            .unwrap();

        HttpResponse::Ok().json(token_data.claims)
    } else {
        HttpResponse::Unauthorized().body("Not authenticated")
    }
}
