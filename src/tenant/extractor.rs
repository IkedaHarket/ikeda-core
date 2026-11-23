use super::context::{AuthSource, TenantContext};
use crate::error::AppError;
use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};
use uuid::Uuid;

/// Header name constants for multi-tenant extraction.
pub const HEADER_TENANT_ID: &str = "x-tenant-id";
pub const HEADER_USER_ID: &str = "x-user-id";
pub const HEADER_SCOPES: &str = "x-scopes";
pub const HEADER_API_KEY: &str = "x-api-key";

impl<S> FromRequestParts<S> for TenantContext
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Check for explicit X-Tenant-Id header (Mock / Dev / Gateway mode)
        if let Some(tenant_header) = parts.headers.get(HEADER_TENANT_ID) {
            let tenant_str = tenant_header.to_str().map_err(|_| {
                AppError::BadRequest("Header 'X-Tenant-Id' contains invalid characters".to_string())
            })?;

            let tenant_id = Uuid::parse_str(tenant_str.trim()).map_err(|_| {
                AppError::BadRequest(format!(
                    "Header 'X-Tenant-Id' contains an invalid UUID: '{tenant_str}'"
                ))
            })?;

            // Optional user ID header
            let user_id = if let Some(user_header) = parts.headers.get(HEADER_USER_ID) {
                let user_str = user_header.to_str().map_err(|_| {
                    AppError::BadRequest(
                        "Header 'X-User-Id' contains invalid characters".to_string(),
                    )
                })?;
                let uid = Uuid::parse_str(user_str.trim()).map_err(|_| {
                    AppError::BadRequest(format!(
                        "Header 'X-User-Id' contains an invalid UUID: '{user_str}'"
                    ))
                })?;
                Some(uid)
            } else {
                None
            };

            // Optional scopes header (comma-delimited)
            let scopes = if let Some(scopes_header) = parts.headers.get(HEADER_SCOPES) {
                let scopes_str = scopes_header.to_str().map_err(|_| {
                    AppError::BadRequest(
                        "Header 'X-Scopes' contains invalid characters".to_string(),
                    )
                })?;
                scopes_str
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(ToString::to_string)
                    .collect()
            } else {
                Vec::new()
            };

            // Determine auth source (API Key if present, otherwise DevHeaders)
            let auth_source = if parts.headers.contains_key(HEADER_API_KEY) {
                AuthSource::ApiKey
            } else {
                AuthSource::DevHeaders
            };

            return Ok(Self {
                tenant_id,
                user_id,
                scopes,
                auth_source,
            });
        }

        // 2. Check for Authorization Bearer header (Structure ready for JWT / JWKS)
        if let Some(auth_header) = parts.headers.get(header::AUTHORIZATION) {
            let auth_str = auth_header.to_str().map_err(|_| {
                AppError::Unauthorized(
                    "Authorization header contains invalid characters".to_string(),
                )
            })?;

            if let Some(_token) = auth_str.strip_prefix("Bearer ") {
                // In future phases: validate JWT against JWKS and extract claims (tenant_id, sub, scopes)
                return Err(AppError::Unauthorized(
                    "JWT bearer validation via JWKS is not yet configured. Please provide 'X-Tenant-Id' header for development/testing.".to_string(),
                ));
            }

            return Err(AppError::Unauthorized(
                "Invalid Authorization scheme: expected 'Bearer <token>'".to_string(),
            ));
        }

        // 3. Check for API Key without explicit tenant header
        if parts.headers.contains_key(HEADER_API_KEY) {
            return Err(AppError::Unauthorized(
                "API Key validation is not yet configured. Please provide 'X-Tenant-Id' header for development/testing.".to_string(),
            ));
        }

        // 4. Missing required authentication context
        Err(AppError::Unauthorized(
            "Missing tenant authentication context: 'X-Tenant-Id' header is required".to_string(),
        ))
    }
}
