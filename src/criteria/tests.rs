use super::filter::{Criteria, FilterCondition, FilterGroup, FilterRule};
use super::operator::FilterOperator;
use super::order::{OrderBy, OrderDirection};
use super::pagination::Pagination;
use serde_json::json;

#[test]
fn test_pagination_offset_and_defaults() {
    let p = Pagination::default();
    assert_eq!(p.page, 1);
    assert_eq!(p.limit, 20);
    assert_eq!(p.offset(), 0);

    let p2 = Pagination::new(2, 25);
    assert_eq!(p2.page, 2);
    assert_eq!(p2.limit, 25);
    assert_eq!(p2.offset(), 25);

    let p3 = Pagination::new(3, 50);
    assert_eq!(p3.offset(), 100);

    let p_clamp = Pagination::new(0, 500);
    assert_eq!(p_clamp.page, 1);
    assert_eq!(p_clamp.limit, 100);
}

#[test]
fn test_order_by_sql() {
    let asc = OrderBy::asc("created_at");
    assert_eq!(asc.field, "created_at");
    assert_eq!(asc.direction, OrderDirection::Asc);
    assert_eq!(asc.direction.as_sql(), "ASC");

    let desc = OrderBy::desc("starts_at");
    assert_eq!(desc.direction.as_sql(), "DESC");
}

#[test]
fn test_operator_sql() {
    assert_eq!(FilterOperator::Eq.as_sql_operator(), "=");
    assert_eq!(FilterOperator::Neq.as_sql_operator(), "!=");
    assert_eq!(FilterOperator::Gt.as_sql_operator(), ">");
    assert_eq!(FilterOperator::Gte.as_sql_operator(), ">=");
    assert_eq!(FilterOperator::Lt.as_sql_operator(), "<");
    assert_eq!(FilterOperator::Lte.as_sql_operator(), "<=");
    assert_eq!(FilterOperator::Contains.as_sql_operator(), "ILIKE");
    assert_eq!(FilterOperator::In.as_sql_operator(), "IN");
    assert_eq!(FilterOperator::IsNull.as_sql_operator(), "IS NULL");
    assert!(!FilterOperator::IsNull.requires_value());
    assert!(FilterOperator::Eq.requires_value());
}

#[test]
fn test_recursive_filter_serialization_and_deserialization() {
    // Build: (status = "CONFIRMED" AND (calendar_id = "abc" OR is_public = true))
    let rule_status = FilterRule::from(FilterCondition::eq("status", "CONFIRMED"));
    let rule_cal = FilterRule::from(FilterCondition::eq("calendar_id", "abc"));
    let rule_public = FilterRule::from(FilterCondition::eq("is_public", true));

    let inner_or = FilterGroup::Or(vec![rule_cal, rule_public]);
    let root_and = FilterGroup::And(vec![rule_status, FilterRule::from(inner_or)]);

    let criteria = Criteria::new()
        .with_filter(root_and)
        .with_order_by(vec![OrderBy::asc("starts_at"), OrderBy::desc("created_at")])
        .with_pagination(Pagination::new(1, 15));

    let serialized = serde_json::to_string_pretty(&criteria).expect("serialization failed");

    // Verify JSON keys
    assert!(serialized.contains("\"and\":"));
    assert!(serialized.contains("\"or\":"));
    assert!(serialized.contains("\"status\""));
    assert!(serialized.contains("\"CONFIRMED\""));
    assert!(serialized.contains("\"orderBy\":"));
    assert!(serialized.contains("\"pagination\":"));

    let deserialized: Criteria = serde_json::from_str(&serialized).expect("deserialization failed");
    assert_eq!(deserialized, criteria);
}

#[test]
fn test_deserialize_raw_json_payload() {
    let raw_json = json!({
        "filter": {
            "and": [
                { "field": "title", "operator": "contains", "value": "Team Sync" },
                { "field": "deleted_at", "operator": "isNull", "value": null }
            ]
        },
        "orderBy": [
            { "field": "starts_at", "direction": "asc" }
        ],
        "pagination": {
            "page": 2,
            "limit": 10
        }
    });

    let json_str = raw_json.to_string();
    let criteria: Criteria = serde_json::from_str(&json_str).expect("deserialization failed");

    assert!(criteria.filter.is_some());
    assert_eq!(criteria.order_by.as_ref().map(|o| o.len()), Some(1));
    assert_eq!(criteria.pagination.map(|p| p.page), Some(2));
    assert_eq!(criteria.pagination.map(|p| p.offset()), Some(10));
}
