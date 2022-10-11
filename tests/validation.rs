use axum_boilerplate::validation::{email, password, title};
#[test]
fn email_is_trimmed_and_normalized() {
    assert_eq!(email("  Mert@Example.COM ").unwrap(), "mert@example.com");
    for invalid in [
        "",
        "no-at",
        "a@@b.com",
        "a b@example.com",
        "a@",
        "@example.com",
    ] {
        assert!(email(invalid).is_err(), "{invalid}");
    }
}
#[test]
fn passwords_have_byte_bounds_without_silent_trimming() {
    assert!(password("short").is_err());
    assert!(password(&"a".repeat(129)).is_err());
    assert!(password(" correct horse battery ").is_ok());
}
#[test]
fn ticket_titles_are_trimmed_and_unicode_aware() {
    assert_eq!(title("  hello  ").unwrap(), "hello");
    assert!(title("  ").is_err());
    assert!(title(&"界".repeat(200)).is_ok());
    assert!(title(&"界".repeat(201)).is_err());
}
