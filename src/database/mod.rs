use crate::error::AppError;

pub mod db;
pub mod model;
pub mod queries;
pub mod tx;

fn into_app_error(e: sqlx::Error) -> AppError {
    AppError::DBError(e.to_string())
}
