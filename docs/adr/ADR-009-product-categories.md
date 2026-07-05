# ADR-009 — Structured product categories

**Status:** Accepted  
**Date:** 2026-07-05  
**Phase:** 43

## Context

Products used a free-text `category` column (`VARCHAR(100)`). Portal filtering matched exact strings. Admin had no category management. FoodKing-inspired catalog work requires sortable categories with slugs and images.

## Decision

1. Add `inventory.product_categories` (tenant-scoped, slug unique per tenant, `sort_order`, optional `image_file_id`).
2. Add `inventory.products.category_id` FK; backfill from distinct legacy `category` values in migration `20260705120000_product_categories.sql`.
3. Keep legacy `products.category` column for now (read-only legacy); all writes use `category_id`.
4. Reject legacy `category` string in product create/update API bodies (`400 VALIDATION_ERROR`).
5. Public/portal product list filter `?category=` matches **category slug** via join.
6. Category delete = soft deactivate; assigned products unchanged.

## Consequences

- Phase 44 adds admin category CRUD UI and product category picker.
- Phases 45–46 consume portal category endpoints for catalog UX.
- OpenAPI spec update tracked in Phase 43 follow-up if `pnpm validate:openapi` gaps remain.
