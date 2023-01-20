use axum_boilerplate::db::Database;
#[test]
fn challenges_are_typed_expiring_and_single_use() {
    let db = Database::open(":memory:").unwrap();
    let user = db.create_user("mert@example.com", "hash").unwrap();
    db.issue_challenge(user.id, "verify", "token-one", 100)
        .unwrap();
    assert!(db
        .complete_challenge("token-one", "reset", 1, Some("new-hash"))
        .is_err());
    db.complete_challenge("token-one", "verify", 1, None)
        .unwrap();
    assert!(db.user_by_id(user.id).unwrap().unwrap().verified);
    assert!(db
        .complete_challenge("token-one", "verify", 1, None)
        .is_err());
    db.issue_challenge(user.id, "reset", "expired", 2).unwrap();
    assert!(db
        .complete_challenge("expired", "reset", 2, Some("new-hash"))
        .is_err());
}
#[test]
fn issuing_a_new_challenge_invalidates_old_tokens() {
    let db = Database::open(":memory:").unwrap();
    let user = db.create_user("mert@example.com", "hash").unwrap();
    db.issue_challenge(user.id, "reset", "old", 100).unwrap();
    db.issue_challenge(user.id, "reset", "new", 100).unwrap();
    assert!(db
        .complete_challenge("old", "reset", 1, Some("new-hash"))
        .is_err());
    db.complete_challenge("new", "reset", 1, Some("new-hash"))
        .unwrap();
    assert!(db
        .complete_challenge("new", "reset", 1, Some("other-hash"))
        .is_err());
}
