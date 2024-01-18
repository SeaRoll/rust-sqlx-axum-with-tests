#[derive(Debug)]
pub enum AppError {
    DBError(String),
    ValidationError(String),
}

pub type AppResult<T> = std::result::Result<T, AppError>;
