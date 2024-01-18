use crate::api::schema::UserSchema;
use crate::database::{db, model};
use crate::error::{AppError, AppResult};
use axum::routing::{get, post};
use axum::Router;
use axum::{extract::State, http::StatusCode, Json};

use super::schema::{CreateUserSchema, EmptySuccessSchema, ErrorSchema};
use super::state::AxumAppState;
use super::{mapper, schema};

struct Handler;

impl Handler {
    async fn create_user(dbo: &db::DB, new_user: &schema::CreateUserSchema) -> AppResult<()> {
        let mut tx = dbo.begin().await?;

        let new_user = model::User {
            id: uuid::Uuid::new_v4().to_string(),
            name: new_user.name.clone(),
            email: new_user.email.clone(),
        };

        let result = tx.create_user(&new_user).await;
        if result.is_err() {
            tracing::error!("error: {:?}", result);
            tx.rollback().await?;
            return Err(AppError::DBError(format!("Failed to create user")))?;
        }
        tx.commit().await?;
        Ok(())
    }
    async fn get_users(dbo: &db::DB) -> AppResult<Vec<model::User>> {
        let mut tx = dbo.begin().await?;
        let users = tx.get_users().await;
        if users.is_err() {
            tracing::error!("error: {:?}", users);
            tx.rollback().await?;
            return Err(AppError::DBError(format!("Failed to get users")))?;
        }
        tx.commit().await?;
        Ok(users.unwrap())
    }
}

pub struct API;

impl API {
    pub async fn create_user(
        State(state): State<AxumAppState>,
        Json(payload): Json<CreateUserSchema>,
    ) -> Result<(StatusCode, Json<EmptySuccessSchema>), (StatusCode, Json<ErrorSchema>)> {
        Handler::create_user(&state.db, &payload)
            .await
            .map_err(mapper::into_error_schema)?;
        Ok((
            StatusCode::CREATED,
            Json::from(EmptySuccessSchema { success: true }),
        ))
    }

    pub async fn get_users(
        State(state): State<AxumAppState>,
    ) -> Result<(StatusCode, Json<Vec<UserSchema>>), (StatusCode, Json<ErrorSchema>)> {
        let users = Handler::get_users(&state.db)
            .await
            .map_err(mapper::into_error_schema)?
            .into_iter()
            .map(|u| UserSchema::from(u))
            .collect::<Vec<_>>();
        Ok((StatusCode::OK, Json::from(users)))
    }

    pub fn routes(state: AxumAppState) -> Router {
        let router = Router::new()
            .route("/", post(Self::create_user))
            .route("/", get(Self::get_users))
            .with_state(state);
        router
    }
}
