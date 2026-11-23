use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use utoipa::ToSchema;
use uuid::Uuid;

/// Source of authentication from which the TenantContext was resolved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AuthSource {
    /// Development/Mock headers (e.g. `X-Tenant-Id`, `X-User-Id`, `X-Scopes`).
    DevHeaders,
    /// Standard Bearer JWT token validated via JWKS.
    BearerJwt,
    /// Tenant or service API Key (`X-Api-Key`).
    ApiKey,
}

/// Execution context for multi-tenant requests.
///
/// Holds the resolved `tenant_id` (mandatory for Row-Level Security in PostgreSQL),
/// optional authenticated `user_id`, granted permissions/scopes, and authentication provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct TenantContext {
    /// Unique tenant identifier (used directly in `SET LOCAL app.current_tenant = $1`).
    pub tenant_id: Uuid,
    /// Authenticated user identifier (if request was made on behalf of an authenticated user).
    pub user_id: Option<Uuid>,
    /// List of scopes/permissions granted to this request.
    pub scopes: Vec<String>,
    /// Source mechanism used to authenticate this request.
    pub auth_source: AuthSource,
}

impl TenantContext {
    /// Constructs a basic tenant context with default DevHeaders source.
    #[must_use]
    pub fn new(tenant_id: Uuid) -> Self {
        Self {
            tenant_id,
            user_id: None,
            scopes: Vec::new(),
            auth_source: AuthSource::DevHeaders,
        }
    }

    /// Builder method to attach an authenticated user ID.
    #[must_use]
    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Builder method to attach a list of scopes.
    #[must_use]
    pub fn with_scopes(mut self, scopes: Vec<String>) -> Self {
        self.scopes = scopes;
        self
    }

    /// Builder method to set the authentication source.
    #[must_use]
    pub fn with_auth_source(mut self, auth_source: AuthSource) -> Self {
        self.auth_source = auth_source;
        self
    }

    /// Checks if a given scope is present in the context.
    #[must_use]
    pub fn has_scope(&self, required_scope: &str) -> bool {
        self.scopes.iter().any(|s| s == required_scope || s == "*")
    }

    /// Enforces that a required scope is present, returning `AppError::Forbidden` if missing.
    pub fn require_scope(&self, required_scope: &str) -> Result<(), AppError> {
        if self.has_scope(required_scope) {
            Ok(())
        } else {
            Err(AppError::Forbidden(format!(
                "Missing required scope: '{required_scope}'"
            )))
        }
    }

    /// Returns scopes as a HashSet for fast lookup.
    #[must_use]
    pub fn scopes_set(&self) -> HashSet<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
}
