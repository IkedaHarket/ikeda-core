//! Convenient re-exports of common platform primitives.

pub use crate::criteria::{
    Criteria, FilterCondition, FilterGroup, FilterOperator, FilterRule, OrderBy, OrderDirection,
    Pagination, DEFAULT_LIMIT, DEFAULT_PAGE, MAX_LIMIT,
};
pub use crate::error::{AppError, ValidationErrorDetail};
pub use crate::response::{ApiResponse, PaginatedApiResponse, PaginationMeta};
pub use crate::tenant::{
    AuthSource, TenantContext, HEADER_API_KEY, HEADER_SCOPES, HEADER_TENANT_ID, HEADER_USER_ID,
};
