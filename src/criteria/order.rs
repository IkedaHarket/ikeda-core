use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Sorting direction for query ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum OrderDirection {
    /// Ascending order (lowest/earliest first).
    Asc,
    /// Descending order (highest/latest first).
    Desc,
}

impl OrderDirection {
    /// Returns the SQL keyword for this sorting direction.
    #[must_use]
    pub const fn as_sql(&self) -> &'static str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        }
    }
}

/// Ordering directive specifying a column/field and sorting direction.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub struct OrderBy {
    /// Column or attribute name to sort by.
    pub field: String,
    /// Sorting direction (asc or desc).
    pub direction: OrderDirection,
}

impl OrderBy {
    /// Creates an ascending order rule.
    pub fn asc(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            direction: OrderDirection::Asc,
        }
    }

    /// Creates a descending order rule.
    pub fn desc(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            direction: OrderDirection::Desc,
        }
    }
}
