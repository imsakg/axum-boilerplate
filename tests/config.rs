use axum_boilerplate::config::Config;
use std::collections::HashMap;

#[test]
fn secrets_are_required_and_never_appear_in_debug_output() {
    assert!(Config::from_values(&HashMap::new()).is_err());
    let secret = "correct-horse-battery-staple-32-bytes";
    let values = HashMap::from([("JWT_SECRET".into(), secret.into())]);
    let config = Config::from_values(&values).unwrap();
    assert!(!format!("{config:?}").contains(secret));
    assert_eq!(config.address.to_string(), "127.0.0.1:5353");
    assert_eq!(config.access_ttl, 900);
}
#[test]
fn invalid_address_and_lifetime_fail_at_startup() {
    let mut values = HashMap::from([("JWT_SECRET".into(), "s".repeat(32))]);
    values.insert("APP_ADDR".into(), "localhost:bad".into());
    assert!(Config::from_values(&values).is_err());
    values.remove("APP_ADDR");
    values.insert("ACCESS_TTL_SECONDS".into(), "0".into());
    assert!(Config::from_values(&values).is_err());
}
