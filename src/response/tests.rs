use super::api_response::{ApiResponse, PaginatedApiResponse, PaginationMeta};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MockItem {
    id: u64,
    name: String,
}

#[test]
fn test_api_response_ok() {
    let item = MockItem {
        id: 1,
        name: "Calendar A".to_string(),
    };
    let response = ApiResponse::ok(item);

    assert!(response.success);
    assert!(response.message.is_none());
    assert_eq!(
        response.data,
        Some(MockItem {
            id: 1,
            name: "Calendar A".to_string(),
        })
    );
}

#[test]
fn test_api_response_created() {
    let item = MockItem {
        id: 2,
        name: "New Event".to_string(),
    };
    let response = ApiResponse::created(item, "Resource created successfully");

    assert!(response.success);
    assert_eq!(
        response.message.as_deref(),
        Some("Resource created successfully")
    );
    assert!(response.data.is_some());
}

#[test]
fn test_api_response_message_only() {
    let response = ApiResponse::message("Operation completed");
    assert!(response.success);
    assert_eq!(response.message.as_deref(), Some("Operation completed"));
    assert!(response.data.is_none());
}

#[test]
fn test_pagination_meta_calculations() {
    let meta = PaginationMeta::new(1, 10, 25);
    assert_eq!(meta.page, 1);
    assert_eq!(meta.limit, 10);
    assert_eq!(meta.total_items, 25);
    assert_eq!(meta.total_pages, 3);
    assert!(meta.has_next_page);
    assert!(!meta.has_prev_page);

    let meta_last = PaginationMeta::new(3, 10, 25);
    assert_eq!(meta_last.page, 3);
    assert_eq!(meta_last.total_pages, 3);
    assert!(!meta_last.has_next_page);
    assert!(meta_last.has_prev_page);

    let meta_empty = PaginationMeta::new(1, 10, 0);
    assert_eq!(meta_empty.total_pages, 0);
    assert!(!meta_empty.has_next_page);
    assert!(!meta_empty.has_prev_page);
}

#[test]
fn test_paginated_api_response() {
    let items = vec![
        MockItem {
            id: 1,
            name: "Item 1".to_string(),
        },
        MockItem {
            id: 2,
            name: "Item 2".to_string(),
        },
    ];

    let paginated = PaginatedApiResponse::new(items, 1, 10, 2);
    assert!(paginated.success);
    assert_eq!(paginated.data.len(), 2);
    assert_eq!(paginated.pagination.total_items, 2);
    assert_eq!(paginated.pagination.total_pages, 1);
    assert!(!paginated.pagination.has_next_page);
}

#[test]
fn test_serialization_deserialization() {
    let item = MockItem {
        id: 42,
        name: "Test Serialization".to_string(),
    };
    let response = ApiResponse::ok(item);
    let json = serde_json::to_string(&response).expect("serialization failed");

    assert!(json.contains("\"success\":true"));
    assert!(json.contains("\"timestamp\":"));
    assert!(json.contains("\"data\":{\"id\":42,\"name\":\"Test Serialization\"}"));

    let deserialized: ApiResponse<MockItem> =
        serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(deserialized.data, response.data);
}

#[test]
fn test_into_response() {
    let response = ApiResponse::ok("hello");
    let http_response = response.into_response();
    assert_eq!(http_response.status(), axum::http::StatusCode::OK);
}
