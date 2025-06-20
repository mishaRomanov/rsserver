pub struct Config {
    pub socket_addr: String,
    pub db_addr: String,
}

impl Config {
    pub fn new() -> Self {
        let addr = std::env::var("DB_ADDR").expect("failed to parse DB_ADDR from env");
        // Ill keep this as default for now.
        Self {
            socket_addr: String::from("127.0.0.1:4040"),
            db_addr: addr,
        }
    }
}
