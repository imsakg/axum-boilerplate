use axum_boilerplate::{db::Database, Error};
#[test]
fn refresh_rotation_is_single_use_and_replay_revokes_the_family() {
    let db = Database::open(":memory:").unwrap();
    let user = db.create_user("mert@example.com", "hash").unwrap();
    db.create_session(user.id, "hash-1", "family-1", 100)
        .unwrap();
    assert_eq!(
        db.rotate_session("hash-1", "hash-2", 200, 10).unwrap().0,
        user.id
    );
    assert!(matches!(
        db.rotate_session("hash-1", "hash-3", 200, 11),
        Err(Error::Unauthorized)
    ));
    assert!(matches!(
        db.rotate_session("hash-2", "hash-4", 200, 12),
        Err(Error::Unauthorized)
    ));
}
#[test]
fn expired_refresh_tokens_cannot_be_rotated() {
    let db = Database::open(":memory:").unwrap();
    let user = db.create_user("mert@example.com", "hash").unwrap();
    db.create_session(user.id, "expired", "family", 10).unwrap();
    assert!(db.rotate_session("expired", "new", 100, 10).is_err());
    assert!(db.rotate_session("missing", "new", 100, 1).is_err());
}
