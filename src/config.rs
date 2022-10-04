//! Explicit environment configuration. Secrets are deliberately omitted from Debug.
use std::{collections::HashMap, net::SocketAddr};

#[derive(Clone)]
pub struct Config {
    pub address: SocketAddr,
    pub database_path: String,
    pub jwt_secret: String,
    pub issuer: String,
    pub audience: String,
    pub access_ttl: u64,
    pub refresh_ttl: u64,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("address", &self.address)
            .field("database_path", &self.database_path)
            .field("jwt_secret", &"[REDACTED]")
            .finish()
    }
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Self::from_values(&std::env::vars().collect())
    }
    pub fn from_values(values: &HashMap<String, String>) -> Result<Self, String> {
        let value =
            |key: &str, default: &str| values.get(key).cloned().unwrap_or_else(|| default.into());
        let jwt_secret = value("JWT_SECRET", "");
        if jwt_secret.len() < 32 {
            return Err("JWT_SECRET must contain at least 32 bytes".into());
        }
        let address = value("APP_ADDR", "127.0.0.1:5353")
            .parse()
            .map_err(|_| "APP_ADDR must be an IP socket address")?;
        let ttl = |key: &str, default: u64, min, max| -> Result<u64, String> {
            let n = value(key, &default.to_string())
                .parse::<u64>()
                .map_err(|_| format!("{key} must be an integer"))?;
            if !(min..=max).contains(&n) {
                return Err(format!("{key} is outside its permitted range"));
            }
            Ok(n)
        };
        let database_path = value("DATABASE_PATH", "app.sqlite3");
        let issuer = value("JWT_ISSUER", "axum-boilerplate");
        let audience = value("JWT_AUDIENCE", "axum-api");
        if database_path.trim().is_empty() || issuer.trim().is_empty() || audience.trim().is_empty()
        {
            return Err("Database path, issuer and audience must not be empty".into());
        }
        Ok(Self {
            address,
            database_path,
            jwt_secret,
            issuer,
            audience,
            access_ttl: ttl("ACCESS_TTL_SECONDS", 900, 60, 3600)?,
            refresh_ttl: ttl("REFRESH_TTL_SECONDS", 604800, 3600, 2592000)?,
        })
    }
}
