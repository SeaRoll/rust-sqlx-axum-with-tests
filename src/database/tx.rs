use sqlx::{PgConnection, Postgres};

use crate::error::AppResult;

use super::{into_app_error, model::User, queries};

pub struct TX {
    pub tx: sqlx::Transaction<'static, Postgres>,
}

impl TX {
    pub async fn commit(self) -> AppResult<()> {
        self.tx.commit().await.map_err(into_app_error)
    }

    pub async fn rollback(self) -> AppResult<()> {
        self.tx.rollback().await.map_err(into_app_error)
    }

    pub async fn create_user(&mut self, user: &User) -> AppResult<()> {
        sqlx::query(queries::CREATE_USER)
            .bind(&user.id)
            .bind(&user.name)
            .bind(&user.email)
            .execute(&mut self.tx as &mut PgConnection)
            .await
            .map_err(into_app_error)?;
        Ok(())
    }

    pub async fn get_users(&mut self) -> AppResult<Vec<User>> {
        let users = sqlx::query_as::<_, User>(queries::GET_USERS)
            .fetch_all(&mut self.tx as &mut PgConnection)
            .await
            .map_err(into_app_error)?;
        Ok(users)
    }
}
