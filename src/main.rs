#[macro_use]
extern crate dotenv_codegen;

pub mod api;
#[cfg(test)]
pub mod api_test;
pub mod config;
pub mod database;
pub mod error;

#[tokio::main]
async fn main() {
    // run it with hyper on localhost:3000
    let state = api::state::State::new().await;
    let server_url = format!("0.0.0.0:{}", config::PORT);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    let app = api::API::init(state).await;
    axum::serve(listener, app).await.unwrap();
}
