use super::schema::ErrorSchema;
use axum::{http::StatusCode, Json};

pub fn into_error_schema(error: crate::error::AppError) -> (StatusCode, Json<ErrorSchema>) {
    match error {
        crate::error::AppError::DBError(val) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json::from(ErrorSchema {
                error: "DatabaseError".to_string(),
                message: val.to_string(),
                status: 500,
            }),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json::from(ErrorSchema {
                error: format!("Unknown Error"),
                message: format!("Unknown Error"),
                status: 500,
            }),
        ),
    }
}
