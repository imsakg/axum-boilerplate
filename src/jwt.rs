use crate::{config::Config, crypto, Error, Result};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Tokens {
    secret: String,
    issuer: String,
    audience: String,
    ttl: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    pub exp: u64,
    pub iat: u64,
    pub jti: String,
    pub kind: String,
    pub version: i64,
}

impl Tokens {
    pub fn new(config: &Config) -> Self {
        Self {
            secret: config.jwt_secret.clone(),
            issuer: config.issuer.clone(),
            audience: config.audience.clone(),
            ttl: config.access_ttl,
        }
    }
    pub fn issue(&self, user_id: i64, version: i64, now: u64) -> Result<String> {
        let claims = Claims {
            sub: user_id.to_string(),
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            exp: now.checked_add(self.ttl).ok_or(Error::Internal)?,
            iat: now,
            jti: uuid::Uuid::new_v4().to_string(),
            kind: "access".into(),
            version,
        };
        jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| Error::Internal)
    }
    pub fn verify(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub", "iat"]);
        validation.leeway = 0;
        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|_| Error::Unauthorized)?
        .claims;
        if claims.kind != "access" || claims.iat > crypto::now() + 30 || claims.exp <= claims.iat {
            return Err(Error::Unauthorized);
        }
        Ok(claims)
    }
}
