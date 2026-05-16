use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};

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


pub fn verify_password(
    password: &str,
    hash: &str
) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_| AppError::InternalServerError)?;

    Ok(
        Argon2::default()
            .verify_password(
                password.as_bytes(), 
                &parsed_hash
            )
            .is_ok()
    ) 
}