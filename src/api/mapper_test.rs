use crate::api::mapper;

#[test]
fn test_db_error() {
    let error = crate::error::AppError::DBError(format!("Record not found"));
    let (status, json) = mapper::into_error_schema(error);
    assert_eq!(status, axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(json.0.error, "DatabaseError");
    assert_eq!(json.0.message, "Record not found");
    assert_eq!(json.0.status, 500);
}

#[test]
fn test_unknown_error() {
    let error = crate::error::AppError::ValidationError(format!("Record not found"));
    let (status, json) = mapper::into_error_schema(error);
    assert_eq!(status, axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(json.0.error, "Unknown Error");
    assert_eq!(json.0.message, "Unknown Error");
    assert_eq!(json.0.status, 500);
}
