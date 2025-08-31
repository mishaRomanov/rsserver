use dotenv;

pub struct Config {
    pub socket_addr: String,
    pub db_addr: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn new() -> Self {
        // Parse all envs from .env to actual environment.
        // Might create some more variables later so it should be useful in the future.
        dotenv::dotenv().ok();

        let addr = std::env::var("DB_ADDR").expect("failed to parse DB_ADDR from env");
        let secret = std::env::var("JWT_SECRET").unwrap_or("default_secret".to_string());
        // Ill keep this as default for now.
        Self {
            socket_addr: String::from("127.0.0.1:4040"),
            db_addr: addr,
            jwt_secret: secret,
        }
    }
}
