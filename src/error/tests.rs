use super::app_error::{AppError, ValidationErrorDetail};
use axum::{http::StatusCode, response::IntoResponse};

#[test]
fn test_error_status_codes() {
    assert_eq!(
        AppError::BadRequest("bad input".into()).status_code(),
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        AppError::Unauthorized("no token".into()).status_code(),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        AppError::Forbidden("access denied".into()).status_code(),
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        AppError::NotFound("item not found".into()).status_code(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        AppError::Conflict("already exists".into()).status_code(),
        StatusCode::CONFLICT
    );
    assert_eq!(
        AppError::UnprocessableEntity {
            message: "invalid".into(),
            errors: vec![]
        }
        .status_code(),
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
        AppError::InternalServerError("internal detail".into()).status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(
        AppError::DatabaseError("connection reset".into()).status_code(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[test]
fn test_internal_error_masks_details_for_client() {
    let internal = AppError::InternalServerError("SECRET_PASSWORD_FAILED".into());
    assert_ne!(internal.client_message(), "SECRET_PASSWORD_FAILED");
    assert!(internal
        .client_message()
        .contains("unexpected internal error"));

    let db_err = AppError::DatabaseError("FATAL: relation \"users\" does not exist".into());
    assert!(!db_err.client_message().contains("relation"));
}

#[test]
fn test_validation_error_into_response() {
    let err = AppError::validation(
        "Validation failed",
        vec![
            ValidationErrorDetail::new("email", "Must be a valid email").with_code("invalid_email"),
            ValidationErrorDetail::new("starts_at", "Date cannot be in the past"),
        ],
    );

    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[test]
fn test_error_into_response_status() {
    let not_found = AppError::NotFound("Event 123 not found".into());
    let res = not_found.into_response();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
