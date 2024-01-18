use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::error::AppResult;

use super::{into_app_error, tx::TX};

pub struct DB {
    pool: Pool<Postgres>,
}

impl DB {
    pub async fn new(uri: &str) -> DB {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(uri)
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        DB { pool }
    }

    pub async fn begin(&self) -> AppResult<TX> {
        let tx = self.pool.begin().await.map_err(into_app_error)?;
        Ok(TX { tx })
    }
}
