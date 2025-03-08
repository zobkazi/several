use sqlx::FromRow;
use bcrypt::{hash, verify};

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    // Hash the user's password
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, 4) // cost = 4, higher values mean more secure but slower
    }

    // Verify password against the hash
    pub fn verify_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, &self.password_hash)
    }
}
