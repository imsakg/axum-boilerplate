use axum_boilerplate::crypto::{fingerprint, hash_password, opaque_token, verify_password};

#[test]
fn hashes_are_salted_and_verify_only_the_correct_password() {
    let a = hash_password("a correct horse battery").unwrap();
    let b = hash_password("a correct horse battery").unwrap();
    assert_ne!(a, b);
    assert!(a.starts_with("$argon2id$"));
    assert!(verify_password("a correct horse battery", &a));
    assert!(!verify_password("another password", &a));
    assert!(!verify_password("anything", "broken hash"));
}

#[test]
fn opaque_tokens_are_unique_and_only_fingerprints_are_persisted() {
    let a = opaque_token();
    let b = opaque_token();
    assert_ne!(a, b);
    assert_eq!(a.len(), 64);
    assert_eq!(fingerprint(&a).len(), 64);
    assert_ne!(a, fingerprint(&a));
    assert_eq!(fingerprint(&a), fingerprint(&a));
}
