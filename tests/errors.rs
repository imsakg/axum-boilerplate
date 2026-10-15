use axum::{http::StatusCode, response::IntoResponse};
use axum_boilerplate::Error;

#[test]
fn invalid_login_is_unauthorized_not_an_internal_error() {
    assert_eq!(
        Error::LoginFail.into_response().status(),
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn missing_ticket_uses_a_stable_json_error() {
    let response = Error::TicketDeleteFailIdNotFound { id: 7 }.into_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&bytes).unwrap()["error"]["code"],
        "not_found"
    );
}
