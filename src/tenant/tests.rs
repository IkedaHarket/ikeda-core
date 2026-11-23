use super::context::{AuthSource, TenantContext};
use super::extractor::{HEADER_API_KEY, HEADER_SCOPES, HEADER_TENANT_ID, HEADER_USER_ID};
use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{header, Request},
};
use uuid::Uuid;

#[test]
fn test_tenant_context_scopes() {
    let tenant_id = Uuid::new_v4();
    let ctx = TenantContext::new(tenant_id)
        .with_scopes(vec!["events:read".into(), "events:write".into()]);

    assert!(ctx.has_scope("events:read"));
    assert!(ctx.has_scope("events:write"));
    assert!(!ctx.has_scope("events:delete"));

    assert!(ctx.require_scope("events:read").is_ok());
    assert!(ctx.require_scope("events:delete").is_err());
}

#[test]
fn test_tenant_context_wildcard_scope() {
    let tenant_id = Uuid::new_v4();
    let ctx = TenantContext::new(tenant_id).with_scopes(vec!["*".into()]);

    assert!(ctx.has_scope("events:delete"));
    assert!(ctx.has_scope("admin:all"));
    assert!(ctx.require_scope("anything").is_ok());
}

#[tokio::test]
async fn test_extractor_with_valid_dev_headers() {
    let tenant_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let req = Request::builder()
        .header(HEADER_TENANT_ID, tenant_id.to_string())
        .header(HEADER_USER_ID, user_id.to_string())
        .header(HEADER_SCOPES, "events:read, events:write, calendars:read")
        .body(Body::empty())
        .expect("request build failed");

    let (mut parts, _) = req.into_parts();
    let ctx = TenantContext::from_request_parts(&mut parts, &())
        .await
        .expect("extractor failed");

    assert_eq!(ctx.tenant_id, tenant_id);
    assert_eq!(ctx.user_id, Some(user_id));
    assert_eq!(ctx.auth_source, AuthSource::DevHeaders);
    assert_eq!(
        ctx.scopes,
        vec!["events:read", "events:write", "calendars:read"]
    );
}

#[tokio::test]
async fn test_extractor_with_api_key_header() {
    let tenant_id = Uuid::new_v4();

    let req = Request::builder()
        .header(HEADER_TENANT_ID, tenant_id.to_string())
        .header(HEADER_API_KEY, "test-api-key-123")
        .body(Body::empty())
        .expect("request build failed");

    let (mut parts, _) = req.into_parts();
    let ctx = TenantContext::from_request_parts(&mut parts, &())
        .await
        .expect("extractor failed");

    assert_eq!(ctx.tenant_id, tenant_id);
    assert_eq!(ctx.auth_source, AuthSource::ApiKey);
}

#[tokio::test]
async fn test_extractor_missing_headers() {
    let req = Request::builder()
        .body(Body::empty())
        .expect("request build failed");

    let (mut parts, _) = req.into_parts();
    let result = TenantContext::from_request_parts(&mut parts, &()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_extractor_invalid_tenant_uuid() {
    let req = Request::builder()
        .header(HEADER_TENANT_ID, "not-a-valid-uuid")
        .body(Body::empty())
        .expect("request build failed");

    let (mut parts, _) = req.into_parts();
    let result = TenantContext::from_request_parts(&mut parts, &()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_extractor_bearer_jwt_unconfigured_error() {
    let req = Request::builder()
        .header(header::AUTHORIZATION, "Bearer some-fake-jwt-token")
        .body(Body::empty())
        .expect("request build failed");

    let (mut parts, _) = req.into_parts();
    let result = TenantContext::from_request_parts(&mut parts, &()).await;

    assert!(result.is_err());
}
