use std::sync::Arc;

use crate::{config, database::db};

pub struct State {
    pub db: db::DB,
}

pub type AppState = Arc<State>;

impl State {
    pub async fn new() -> AppState {
        Arc::new(State {
            db: db::DB::new(&config::DATABASE_URL).await,
        })
    }
}
