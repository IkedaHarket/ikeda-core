use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Default page index (1-based).
pub const DEFAULT_PAGE: u64 = 1;
/// Default page size.
pub const DEFAULT_LIMIT: u64 = 20;
/// Hard limit to protect against oversized database queries.
pub const MAX_LIMIT: u64 = 100;

/// Request pagination specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Pagination {
    /// 1-based page number (defaults to 1).
    #[serde(default = "default_page")]
    pub page: u64,
    /// Number of elements per page (defaults to 20, max 100).
    #[serde(default = "default_limit")]
    pub limit: u64,
}

const fn default_page() -> u64 {
    DEFAULT_PAGE
}

const fn default_limit() -> u64 {
    DEFAULT_LIMIT
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: DEFAULT_PAGE,
            limit: DEFAULT_LIMIT,
        }
    }
}

impl Pagination {
    /// Creates a new pagination request clamping limit to valid bounds.
    #[must_use]
    pub fn new(page: u64, limit: u64) -> Self {
        let safe_page = page.max(1);
        let safe_limit = limit.clamp(1, MAX_LIMIT);
        Self {
            page: safe_page,
            limit: safe_limit,
        }
    }

    /// Computes the zero-based SQL `OFFSET`.
    #[must_use]
    pub fn offset(&self) -> u64 {
        (self.page.max(1) - 1) * self.limit.min(MAX_LIMIT)
    }

    /// Returns the effective SQL `LIMIT`.
    #[must_use]
    pub fn sql_limit(&self) -> u64 {
        self.limit.clamp(1, MAX_LIMIT)
    }
}
