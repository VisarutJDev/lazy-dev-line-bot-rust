pub struct Config {
    pub port: u16,
    pub host: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap(),
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        }
    }
}