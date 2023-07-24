pub struct Config {
    pub base_url: String,
}

impl Config {
    pub fn new() -> Self {
        let base_url = "http://localhost:3000".to_owned();

        Self { base_url }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
