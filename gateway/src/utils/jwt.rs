use jsonwebtoken::{
    encode,
    decode,
    Header,
    Validation,
    EncodingKey,
    DecodingKey,
};

use chrono::{Duration, Utc};

use serde::{Serialize, Deserialize};

use crate::utils;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}


pub fn create_jwt(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = (*utils::constants::JWT_SECRET).clone();


    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration
    };

    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(secret.as_ref()),
    )
}


pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = (*utils::constants::JWT_SECRET).clone();

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )?;

    Ok(data.claims)
}
