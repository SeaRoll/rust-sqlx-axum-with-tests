use axum::http::StatusCode;
use axum_test::TestServer;
use serde_json::json;
use testcontainers::clients;

use crate::api;

#[tokio::test]
async fn get_users_empty() {
    let docker = clients::Cli::default();
    let state = api::state::State::new().await;
    let app = api::API::init(state.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/users").await;
    let response_two = server
        .post("/users")
        .json(&json!({
            "name": "test",
            "email": "test@gmail.com"
        }))
        .await;
    assert_eq!(StatusCode::OK, response.status_code());
    assert_eq!(StatusCode::CREATED, response_two.status_code());
}
