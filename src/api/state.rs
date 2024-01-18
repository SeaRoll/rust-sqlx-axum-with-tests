use std::sync::Arc;

use crate::{config, database::db};

pub struct AppState {
    pub db: db::DB,
}

pub type AxumAppState = Arc<AppState>;

impl AppState {
    pub async fn new(custom_db_uri: Option<String>) -> AxumAppState {
        let uri = custom_db_uri.unwrap_or(config::DATABASE_URL.to_string());
        Arc::new(AppState {
            db: db::DB::new(&uri).await,
        })
    }
}
