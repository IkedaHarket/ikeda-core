use super::operator::FilterOperator;
use super::order::OrderBy;
use super::pagination::Pagination;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Single atomic filter condition (field OPERATOR value).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct FilterCondition {
    /// Target field or column name.
    pub field: String,
    /// Comparison operator.
    pub operator: FilterOperator,
    /// Comparison operand value.
    #[schema(value_type = Object)]
    pub value: serde_json::Value,
}

impl FilterCondition {
    /// Creates a new filter condition.
    pub fn new(
        field: impl Into<String>,
        operator: FilterOperator,
        value: impl Into<serde_json::Value>,
    ) -> Self {
        Self {
            field: field.into(),
            operator,
            value: value.into(),
        }
    }

    /// Convenience helper for equality conditions: `field = value`.
    pub fn eq(field: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        Self::new(field, FilterOperator::Eq, value)
    }

    /// Convenience helper for inequality conditions: `field != value`.
    pub fn neq(field: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        Self::new(field, FilterOperator::Neq, value)
    }

    /// Convenience helper for contains conditions: `field ILIKE %value%`.
    pub fn contains(field: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        Self::new(field, FilterOperator::Contains, value)
    }
}

/// Recursive grouping of filter rules combined via AND or OR logic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum FilterGroup {
    /// All nested rules must match (conjunction).
    #[serde(rename = "and")]
    And(Vec<FilterRule>),
    /// At least one nested rule must match (disjunction).
    #[serde(rename = "or")]
    Or(Vec<FilterRule>),
}

/// Either an atomic condition or an arbitrarily nested group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum FilterRule {
    /// Direct filter condition.
    Condition(FilterCondition),
    /// Nested conjunction or disjunction group.
    #[schema(value_type = Object)]
    Group(Box<FilterGroup>),
}

impl From<FilterCondition> for FilterRule {
    fn from(condition: FilterCondition) -> Self {
        Self::Condition(condition)
    }
}

impl From<FilterGroup> for FilterRule {
    fn from(group: FilterGroup) -> Self {
        Self::Group(Box::new(group))
    }
}

/// Aggregate criteria specification for complex dynamic queries.
///
/// Encapsulates recursive filtering, multi-field ordering, and pagination.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Criteria {
    /// Root filter expression (AND or OR group).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterGroup>,
    /// Ordering instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<OrderBy>>,
    /// Page and size pagination parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

impl Criteria {
    /// Creates an empty criteria specification.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Attaches a root filter group.
    #[must_use]
    pub fn with_filter(mut self, filter: FilterGroup) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Attaches ordering rules.
    #[must_use]
    pub fn with_order_by(mut self, order_by: Vec<OrderBy>) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// Attaches pagination.
    #[must_use]
    pub fn with_pagination(mut self, pagination: Pagination) -> Self {
        self.pagination = Some(pagination);
        self
    }
}
