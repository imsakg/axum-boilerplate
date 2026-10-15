use axum_boilerplate::{db::Database, Error};
#[test]
fn ticket_repository_preserves_ownership_and_validates_titles() {
    let db = Database::open(":memory:").unwrap();
    let a = db.create_user("a@example.com", "hash").unwrap();
    let b = db.create_user("b@example.com", "hash").unwrap();
    let ticket = db.create_ticket(a.id, "  First ticket  ").unwrap();
    assert_eq!(ticket.title, "First ticket");
    assert_eq!(ticket.status, "open");
    assert_eq!(db.ticket(a.id, ticket.id).unwrap().id, ticket.id);
    assert!(matches!(db.ticket(b.id, ticket.id), Err(Error::NotFound)));
    assert!(db.create_ticket(a.id, " ").is_err());
}
