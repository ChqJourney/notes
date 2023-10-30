#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub secret_key: String,
    pub jwt_expires_in: i64,
    pub refresh_expires_in: i64,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let secret_key = std::env::var("SECRET_KEY").expect("Secret key must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let refresh_expires_in = std::env::var("REFRESH_EXPIRED_IN").expect("JWT_MAXAGE must be set");
        println!("{database_url}");
        Config {
            database_url,
            secret_key,
            jwt_expires_in:jwt_expires_in.parse::<i64>().unwrap(),
            refresh_expires_in:refresh_expires_in.parse::<i64>().unwrap()
        }
    }
}