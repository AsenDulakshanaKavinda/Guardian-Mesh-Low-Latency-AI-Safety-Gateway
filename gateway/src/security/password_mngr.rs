use crate::utils::errors::AppError;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};


pub fn get_hashed_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::HashingError)?
        .to_string();

    Ok(hashed_password)
}


pub fn verify_password(input_password: &str, hashed_password: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hashed_password).map_err(|_| AppError::HashingError)?;

    let argon2 = Argon2::default();

    match argon2.verify_password(input_password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
