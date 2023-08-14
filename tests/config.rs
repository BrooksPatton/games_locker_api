pub struct TestConfig {
    pub base_url: String,
}

impl TestConfig {
    pub fn new() -> Self {
        let base_url = "http://localhost:3000";

        Self {
            base_url: base_url.to_owned(),
        }
    }
}

impl Default for TestConfig {
    fn default() -> Self {
        Self::new()
    }
}
