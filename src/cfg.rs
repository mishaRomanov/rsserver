pub struct Config {
    pub socket_addr: String,
}

impl Config {
    pub fn new() -> Self {
        // Ill keep this as default for now.
        Self {
            socket_addr: String::from("127.0.0.1:4040"),
        }
    }
}
