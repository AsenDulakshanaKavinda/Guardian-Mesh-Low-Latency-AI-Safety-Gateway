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


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

const JWT_SECRET: &[u8] = b"super_secret_key_change_this";

pub fn create_jwt(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    // let token = (*utils::constants::AUTH_TOKEN).clone();


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
        &EncodingKey::from_secret(JWT_SECRET),
    )
}


pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default()
    )?;

    Ok(data.claims)
}
