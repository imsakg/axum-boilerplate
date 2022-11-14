use axum_boilerplate::{db::Database, Error};
#[test]
fn user_repository_enforces_uniqueness_and_hides_passwords() {
    let db = Database::open(":memory:").unwrap();
    let user = db
        .create_user("mert@example.com", "not-a-real-hash")
        .unwrap();
    assert!(user.id > 0);
    assert_eq!(
        db.user_by_id(user.id).unwrap().unwrap().email,
        "mert@example.com"
    );
    assert!(matches!(
        db.create_user("MERT@example.com", "hash"),
        Err(Error::Conflict)
    ));
    let json = serde_json::to_value(&user).unwrap();
    assert!(json.get("password_hash").is_none());
    assert!(json.get("token_version").is_none());
    assert!(db.user_by_email("missing@example.com").unwrap().is_none());
}
