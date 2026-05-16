use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref DATABASE_URL: String = set_database();
    pub static ref AUTH_TOKEN: String = set_auth_token();
}

fn set_database() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}


fn set_auth_token() -> String {
    dotenv().ok();
    env::var("AUTH_TOKEN").unwrap()
}
