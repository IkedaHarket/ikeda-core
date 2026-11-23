# ikeda-core

`ikeda-core` es la librería central y compartida por todos los microservicios y APIs del ecosistema `ikedadev-platform`. Provee tipos estandarizados, manejo de errores homogéneo, extracción de contexto multi-tenant y un árbol dinámico de filtrado tipo Criteria Pattern.

## Módulos y Capacidades

### 1. `response` (`ApiResponse<T>` y `PaginatedApiResponse<T>`)
- Envoltorio estándar para respuestas de éxito o error con timestamp UTC ISO-8601.
- `PaginatedApiResponse<T>` con `PaginationMeta` (`page`, `limit`, `total_items`, `total_pages`, `has_next_page`, `has_prev_page`).
- Compatible con `utoipa::ToSchema` para generación de especificaciones OpenAPI 3.0 / 3.1.
- Implementa `IntoResponse` de Axum para integración directa en handlers.

### 2. `error` (`AppError`)
- Manejo centralizado de errores basado en `thiserror`.
- Mapeo estricto a códigos HTTP:
  - `BadRequest(String)` -> `400 Bad Request`
  - `Unauthorized(String)` -> `401 Unauthorized`
  - `Forbidden(String)` -> `403 Forbidden`
  - `NotFound(String)` -> `404 Not Found`
  - `Conflict(String)` -> `409 Conflict`
  - `UnprocessableEntity { message, errors }` -> `422 Unprocessable Entity`
  - `InternalServerError(String)` y `DatabaseError(String)` -> `500 Internal Server Error` (ocultando detalles sensibles al cliente y registrando trazas vía `tracing::error!`).
- Implementa `IntoResponse` retornando la misma estructura de `ApiResponse`.

### 3. `tenant` (`TenantContext` y Extractor Axum)
- Representa el contexto de ejecución multi-inquilino (`tenant_id: Uuid`, `user_id: Option<Uuid>`, `scopes: Vec<String>`, `auth_source: AuthSource`).
- Extractor `FromRequestParts` de Axum:
  - Lee cabeceras de desarrollo / gateway: `X-Tenant-Id`, `X-User-Id`, `X-Scopes`, `X-Api-Key`.
  - Estructurado para interceptar tokens JWT Bearer vía JWKS en fases posteriores.

### 4. `criteria` (Criteria Pattern Dinámico)
- Estructuras serializables (`serde` + `utoipa`) para consultas complejas:
  - `FilterCondition`: `field`, `operator` (`eq`, `neq`, `gt`, `gte`, `lt`, `lte`, `contains`, `in`, `isNull`, etc.), `value`.
  - `FilterGroup`: combinación recursiva mediante `and` y `or`.
  - `OrderBy`: especificación de ordenamiento (`field`, `direction: asc | desc`).
  - `Pagination`: paginación delimitada con cálculo de `offset()` y límites de seguridad (`MAX_LIMIT = 100`).

## Verificación de Calidad

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```
