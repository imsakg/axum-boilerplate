use crate::{config::Config, db::Database, jwt::Tokens, Error, Result};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: Database,
    pub(crate) tokens: Tokens,
}
impl AppState {
    pub fn new(config: Config, db: Database) -> Self {
        let tokens = Tokens::new(&config);
        Self { config, db, tokens }
    }
    /// Hashing and synchronous SQLite work stay off Tokio reactor threads.
    pub async fn run<T, F>(&self, work: F) -> Result<T>
    where
        T: Send + 'static,
        F: FnOnce(Self) -> Result<T> + Send + 'static,
    {
        let state = self.clone();
        tokio::task::spawn_blocking(move || work(state))
            .await
            .map_err(|_| Error::Internal)?
    }
}
