use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Standard metadata for paginated responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    /// Current 1-based page index.
    pub page: u64,
    /// Number of items requested per page.
    pub limit: u64,
    /// Total number of items matching the query.
    pub total_items: u64,
    /// Total number of pages available.
    pub total_pages: u64,
    /// Whether there is a subsequent page.
    pub has_next_page: bool,
    /// Whether there is a preceding page.
    pub has_prev_page: bool,
}

impl PaginationMeta {
    /// Calculates pagination metadata from page, limit and total items count.
    #[must_use]
    pub fn new(page: u64, limit: u64, total_items: u64) -> Self {
        let safe_limit = limit.max(1);
        let safe_page = page.max(1);
        let total_pages = total_items.div_ceil(safe_limit);
        let has_next_page = safe_page < total_pages;
        let has_prev_page = safe_page > 1 && total_pages > 0;

        Self {
            page: safe_page,
            limit: safe_limit,
            total_items,
            total_pages,
            has_next_page,
            has_prev_page,
        }
    }
}

/// Standardized API envelope for all single-item responses and operations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    /// Indicates whether the operation succeeded.
    pub success: bool,
    /// Optional human-readable message describing the outcome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Payload containing the response data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// UTC timestamp formatted in ISO-8601 when response was generated.
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    /// Creates a successful response with the given data.
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            message: None,
            data: Some(data),
            timestamp: Utc::now(),
        }
    }

    /// Creates a successful response with data and a custom message (e.g. for resource creation).
    pub fn created(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            data: Some(data),
            timestamp: Utc::now(),
        }
    }

    /// Creates an error response wrapping error data/details.
    pub fn error_with_data(message: impl Into<String>, data: T) -> Self {
        Self {
            success: false,
            message: Some(message.into()),
            data: Some(data),
            timestamp: Utc::now(),
        }
    }

    /// Converts this response into an Axum HTTP `Response` with the specified `StatusCode`.
    pub fn into_response_with_status(self, status: StatusCode) -> Response
    where
        T: Serialize,
    {
        (status, Json(self)).into_response()
    }
}

impl ApiResponse<()> {
    /// Creates a successful response carrying only an informational message.
    pub fn message(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            data: None,
            timestamp: Utc::now(),
        }
    }

    /// Creates an error response carrying only an error message.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: Some(message.into()),
            data: None,
            timestamp: Utc::now(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Standardized API envelope for paginated collection responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PaginatedApiResponse<T> {
    /// Indicates whether the operation succeeded.
    pub success: bool,
    /// Optional human-readable message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// List of items belonging to the current page.
    pub data: Vec<T>,
    /// Pagination metadata.
    pub pagination: PaginationMeta,
    /// UTC timestamp formatted in ISO-8601 when response was generated.
    pub timestamp: DateTime<Utc>,
}

impl<T> PaginatedApiResponse<T> {
    /// Constructs a paginated response calculating metadata from raw metrics.
    pub fn new(data: Vec<T>, page: u64, limit: u64, total_items: u64) -> Self {
        let pagination = PaginationMeta::new(page, limit, total_items);
        Self {
            success: true,
            message: None,
            data,
            pagination,
            timestamp: Utc::now(),
        }
    }

    /// Constructs a paginated response using pre-calculated metadata.
    pub fn with_pagination(data: Vec<T>, pagination: PaginationMeta) -> Self {
        Self {
            success: true,
            message: None,
            data,
            pagination,
            timestamp: Utc::now(),
        }
    }

    /// Attaches an informational message to the paginated response.
    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl<T: Serialize> IntoResponse for PaginatedApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
