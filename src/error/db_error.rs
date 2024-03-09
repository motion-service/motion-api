use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum DbError {


    #[error("MongoDB error")]
    MongoError(#[from] mongodb::error::Error),
    #[error("duplicate key error: {0}")]
    MongoErrorKind(mongodb::error::ErrorKind),
    #[error("duplicate key error: {0}")]
    MongoDuplicateError(mongodb::error::Error),
    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("error serializing BSON")]
    MongoSerializeBsonError(#[from] mongodb::bson::ser::Error),
    #[error("validation error")]
    MongoDataError(#[from] mongodb::bson::document::ValueAccessError),
    #[error("invalid ID: {0}")]
    InvalidIDError(String),
    #[error("Note with ID: {0} not found")]
    NotFoundError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    status: &'static str,
    message: String,
}

impl Into<(axum::http::StatusCode, Json<serde_json::Value>)> for DbError {
    fn into(self) -> (axum::http::StatusCode, Json<serde_json::Value>) {
        let (status, error_response) = match self {
            DbError::MongoErrorKind(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error",
                    message: format!("MongoDB error kind: {}", e),
                },
            ),
            DbError::MongoDuplicateError(_) => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    status: "fail",
                    message: "Note with that title already exists".to_string(),
                },
            ),
            DbError::InvalidIDError(id) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    status: "fail",
                    message: format!("invalid ID: {}", id),
                },
            ),
            DbError::NotFoundError(id) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: "fail",
                    message: format!("Note with ID: {} not found", id),
                },
            ),
            DbError::MongoError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error",
                    message: format!("MongoDB error: {}", e),
                },
            ),
            DbError::MongoQueryError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error",
                    message: format!("MongoDB error: {}", e),
                },
            ),
            DbError::MongoSerializeBsonError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error",
                    message: format!("MongoDB error: {}", e),
                },
            ),
            DbError::MongoDataError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error",
                    message: format!("MongoDB error: {}", e),
                },
            ),
        };
        (status, Json(serde_json::to_value(error_response).unwrap()))
    }
}

impl From<DbError> for (StatusCode, ErrorResponse) {
    fn from(err: DbError) -> (StatusCode, ErrorResponse) {
        err.into()
    }
}
