#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub struct Config {
    api_addr: &'static str,
    todo_addr: &'static str,
}

impl Config {
    pub fn parse() -> Self {
        if let Err(error) = dotenvy::dotenv() {
            tracing::warn!("Failed to load .env: {error:?}");
        }

        let api_addr = dotenvy::var("API_ADDR")
            .expect("environment variable `API_ADDR` not found")
            .leak();

        let todo_addr = dotenvy::var("TODO_ADDR")
            .expect("environment variable `TODO_ADDR` not found")
            .leak();

        Self {
            api_addr,
            todo_addr,
        }
    }

    #[inline]
    pub fn api_addr(&self) -> &'static str {
        self.api_addr
    }

    #[inline]
    pub fn todo_addr(&self) -> &'static str {
        self.todo_addr
    }
}
