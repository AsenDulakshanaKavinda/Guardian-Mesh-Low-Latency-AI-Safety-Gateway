use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};

use crate::utils::errors::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::HashingError)?
        .to_string();

    Ok(password_hash)

}