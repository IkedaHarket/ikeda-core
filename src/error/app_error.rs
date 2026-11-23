use crate::response::ApiResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

/// Granular detail for field-level validation errors (e.g. 422 Unprocessable Entity).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ValidationErrorDetail {
    /// Name or path of the field that failed validation.
    pub field: String,
    /// Explanation of the validation failure.
    pub message: String,
    /// Optional machine-readable validation code (e.g. "email_invalid", "min_length").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl ValidationErrorDetail {
    /// Creates a new validation error detail.
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            code: None,
        }
    }

    /// Attaches an error code to the validation detail.
    #[must_use]
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
}

/// Centralized application errors mapped directly to HTTP status codes and uniform API responses.
#[derive(Debug, Error)]
pub enum AppError {
    /// 400 Bad Request: Malformed request syntax or invalid input parameters.
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// 401 Unauthorized: Authentication is missing or invalid.
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// 403 Forbidden: Authenticated user lacks necessary permissions.
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// 404 Not Found: The requested resource does not exist.
    #[error("Not found: {0}")]
    NotFound(String),

    /// 409 Conflict: Request conflicts with current resource state (e.g. duplicate key, overlap).
    #[error("Conflict: {0}")]
    Conflict(String),

    /// 422 Unprocessable Entity: Syntactically correct but semantically invalid payload.
    #[error("Unprocessable entity: {message}")]
    UnprocessableEntity {
        /// Overview message for the validation failure.
        message: String,
        /// List of specific field-level validation errors.
        errors: Vec<ValidationErrorDetail>,
    },

    /// 500 Internal Server Error: Unexpected server-side failure.
    #[error("Internal server error: {0}")]
    InternalServerError(String),

    /// 500 Internal Server Error wrapping database failures safely without leaking internals.
    #[error("Database error")]
    DatabaseError(String),
}

impl AppError {
    /// Maps the error variant to its corresponding HTTP `StatusCode`.
    #[must_use]
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::InternalServerError(_) | Self::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    /// Returns the client-safe error message.
    #[must_use]
    pub fn client_message(&self) -> String {
        match self {
            Self::BadRequest(msg)
            | Self::Unauthorized(msg)
            | Self::Forbidden(msg)
            | Self::NotFound(msg)
            | Self::Conflict(msg) => msg.clone(),
            Self::UnprocessableEntity { message, .. } => message.clone(),
            Self::InternalServerError(_) | Self::DatabaseError(_) => {
                "An unexpected internal error occurred. Please try again later.".to_string()
            }
        }
    }

    /// Creates an UnprocessableEntity error from field details.
    pub fn validation(
        message: impl Into<String>,
        errors: impl IntoIterator<Item = ValidationErrorDetail>,
    ) -> Self {
        Self::UnprocessableEntity {
            message: message.into(),
            errors: errors.into_iter().collect(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let message = self.client_message();

        // Trace server-side errors with high priority, client errors with warn/debug
        match &self {
            Self::InternalServerError(err) => {
                tracing::error!(status = %status, error = %err, "Internal server error occurred");
            }
            Self::DatabaseError(err) => {
                tracing::error!(status = %status, db_error = %err, "Database error occurred");
            }
            Self::UnprocessableEntity { errors, .. } => {
                tracing::warn!(status = %status, ?errors, "Validation failed");
            }
            client_err => {
                tracing::debug!(status = %status, error = %client_err, "Client error returned");
            }
        }

        let body = match self {
            Self::UnprocessableEntity { errors, .. } => {
                let error_payload = match serde_json::to_value(errors) {
                    Ok(val) => val,
                    Err(_) => serde_json::Value::Null,
                };
                ApiResponse::error_with_data(message, error_payload)
            }
            _ => ApiResponse::error_with_data(message, serde_json::Value::Null),
        };

        (status, Json(body)).into_response()
    }
}
