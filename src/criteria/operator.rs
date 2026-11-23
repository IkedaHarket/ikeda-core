use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Supported operators for dynamic query criteria filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum FilterOperator {
    /// Equal: `=`
    Eq,
    /// Not equal: `!=` or `<>`
    Neq,
    /// Greater than: `>`
    Gt,
    /// Greater than or equal: `>=`
    Gte,
    /// Less than: `<`
    Lt,
    /// Less than or equal: `<=`
    Lte,
    /// String contains substring (or ILIKE / LIKE)
    Contains,
    /// String starts with prefix
    StartsWith,
    /// String ends with suffix
    EndsWith,
    /// In collection of values: `IN (...)`
    #[serde(rename = "in")]
    In,
    /// Not in collection of values: `NOT IN (...)`
    #[serde(rename = "notIn")]
    NotIn,
    /// Target column is null: `IS NULL`
    IsNull,
    /// Target column is not null: `IS NOT NULL`
    IsNotNull,
}

impl FilterOperator {
    /// Returns the standard SQL operator or keyword representation.
    #[must_use]
    pub const fn as_sql_operator(&self) -> &'static str {
        match self {
            Self::Eq => "=",
            Self::Neq => "!=",
            Self::Gt => ">",
            Self::Gte => ">=",
            Self::Lt => "<",
            Self::Lte => "<=",
            Self::Contains => "ILIKE",
            Self::StartsWith => "ILIKE",
            Self::EndsWith => "ILIKE",
            Self::In => "IN",
            Self::NotIn => "NOT IN",
            Self::IsNull => "IS NULL",
            Self::IsNotNull => "IS NOT NULL",
        }
    }

    /// Whether this operator requires an operand value.
    #[must_use]
    pub const fn requires_value(&self) -> bool {
        !matches!(self, Self::IsNull | Self::IsNotNull)
    }
}
