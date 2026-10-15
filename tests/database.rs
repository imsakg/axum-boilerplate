use axum_boilerplate::db::Database;
#[test]
fn migrations_are_idempotent_and_foreign_keys_are_enabled() {
    let db = Database::open(":memory:").unwrap();
    db.migrate().unwrap();
    assert_eq!(db.schema_version().unwrap(), 1);
    assert!(db.foreign_keys_enabled().unwrap());
    assert!(db.ready().is_ok());
}
