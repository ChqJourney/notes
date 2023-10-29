#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub pri_key: String,
    pub pub_key:String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pri_key = std::env::var("PRI_KEY").expect("private key must be set");
        let pub_key=std::env::var("PUB_KEY").expect("public key must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        println!("{database_url}");
        Config {
            database_url,
            pri_key,
            pub_key,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}