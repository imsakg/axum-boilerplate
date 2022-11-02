use axum_boilerplate::{config::Config, jwt::Tokens};
use std::collections::HashMap;
fn config() -> Config {
    Config::from_values(&HashMap::from([("JWT_SECRET".into(), "s".repeat(32))])).unwrap()
}
#[test]
fn jwt_verifies_signature_issuer_audience_and_expiry() {
    let settings = config();
    let tokens = Tokens::new(&settings);
    let token = tokens.issue(7, 2, axum_boilerplate::crypto::now()).unwrap();
    let claims = tokens.verify(&token).unwrap();
    assert_eq!(claims.sub, "7");
    assert_eq!(claims.version, 2);
    assert_eq!(claims.kind, "access");
    assert!(tokens.verify("bad.token.data").is_err());
    assert!(tokens.verify(&tokens.issue(7, 2, 1).unwrap()).is_err());
    let mut changed = settings.clone();
    changed.jwt_secret = "x".repeat(32);
    assert!(Tokens::new(&changed).verify(&token).is_err());
    changed = settings.clone();
    changed.issuer = "other".into();
    assert!(Tokens::new(&changed).verify(&token).is_err());
    changed = settings.clone();
    changed.audience = "other".into();
    assert!(Tokens::new(&changed).verify(&token).is_err());
}
#[test]
fn issued_tokens_have_unique_ids() {
    let tokens = Tokens::new(&config());
    let now = axum_boilerplate::crypto::now();
    assert_ne!(
        tokens.issue(1, 0, now).unwrap(),
        tokens.issue(1, 0, now).unwrap()
    );
}
