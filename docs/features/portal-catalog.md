# Portal catalog components (Phase 45)

> Reusable catalog UI in `apps/portal` (`@full-sales/portal`).  
> API contract: [API-CONTRACT.md](../API-CONTRACT.md) · Backend routes: [ROUTE-MATRIX.md](../ROUTE-MATRIX.md)

**Status:** Component library complete (Phase 45); page rewrite in Phase 46.

---

## Purpose

Professional catalog building blocks for the commerce portal — category bar, product cards (list/grid), toolbar, empty/skeleton states, and category API hooks. Inspired by FoodKing `CategoryComponent` and `ItemComponent`.

---

## Dev commands

```bash
pnpm dev:portal                              # http://127.0.0.1:5175
pnpm --filter @full-sales/portal lint test build
```

---

## Component library (`src/components/catalog/`)

| Component | Role |
|-----------|------|
| `CategoryBar` | Horizontal scroll category chips with thumb + active state |
| `ProductCardGrid` | Vertical product card (cover, title, price, add-to-cart) |
| `ProductCardList` | Horizontal list row card |
| `ProductCatalog` | Composes bar + toolbar + cards; manages list/grid `viewMode` |
| `CatalogToolbar` | Category title, search slot, list/grid toggle |
| `CatalogEmptyState` | Empty catalog with illustration |
| `CatalogSkeleton` | Loading placeholders |
| `ProductImage` | Shared image with icon/initial fallback |

---

## Catalog utilities

| Path | Purpose |
|------|---------|
| `src/lib/catalog/viewMode.ts` | `CatalogViewMode` (`list` \| `grid`) constants |
| `src/lib/catalog/useCatalogCategories.ts` | React Query hook — `staleTime` 5 min (aligned with site settings) |
| `src/styles/theme.css` | `@layer components` — chip active state, card shadows, price typography |

---

## API client

| Function | Endpoint (public / portal) |
|----------|----------------------------|
| `fetchPortalCategories` | `GET /v1/public/categories` or `/v1/portal/categories` |
| `fetchPortalCategoryBySlug` | `GET /v1/public/categories/{slug}` or `/v1/portal/categories/{slug}` |
| `fetchPortalProducts` | Existing — supports `?category=` slug filter |

Types: `PortalCategory`, `PortalCategoryWithProducts` in `src/lib/api/types.ts`.

SSE invalidation (`useCatalogRealtime`) refreshes both `['portal', 'products']` and `['portal', 'categories']`.

---

## i18n keys

`catalog.allCategories`, `catalog.viewList`, `catalog.viewGrid`, `catalog.emptyDescription` (+ existing `catalog.*`).

---

## Testing

| Layer | Command |
|-------|---------|
| Unit + component | `pnpm --filter @full-sales/portal test` |
| Setup | `@testing-library/react`, `happy-dom`, `tests/setup.ts` |

Component tests assert user-visible contracts (active category, price display, layout toggle) — not implementation mirrors.

---

## Phase 46 (next)

- Rewrite `routes/_authenticated/index.tsx` to use `ProductCatalog` + `useCatalogCategories`
- Wire category slug to `fetchPortalProducts({ category })`
- Optional category slug route

---

**Updated:** 2026-07-05 (Phase 45)
