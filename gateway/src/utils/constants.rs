use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref DATABASE_URL: String = set_database();
    pub static ref JWT_SECRET: String = set_jwt_secret();
}

fn set_database() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL missing!")
}


fn set_jwt_secret() -> String {
    dotenv().ok();
    env::var("JWT_SECRET").expect("JWT_SECRET missing!")
}
