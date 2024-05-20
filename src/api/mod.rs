use axum::Router;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub mod mapper;
pub mod schema;
pub mod state;
pub mod user;

#[cfg(test)]
mod mapper_test;

#[cfg(test)]
mod schema_test;

pub struct API;

impl API {
    pub async fn init(state: state::AppState) -> Router {
        // setup tracing
        tracing_subscriber::fmt().compact().init();

        // build our application with a single route
        let app = Router::new()
            .nest("/users", user::API::routes(state.clone()))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            );
        app
    }
}
