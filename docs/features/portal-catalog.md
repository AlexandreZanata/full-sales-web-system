# Portal catalog (Phases 45–47)

> Reusable catalog UI + production catalog page in `apps/portal` (`@full-sales/portal`).  
> API contract: [API-CONTRACT.md](../API-CONTRACT.md) · Backend routes: [ROUTE-MATRIX.md](../ROUTE-MATRIX.md)

**Status:** Phase 47 complete — demo-ready dev seed + documentation.

**Design inspiration:** [FoodKing](https://github.com/inilabs/foodking) catalog UX (`MenuComponent`, `CategoryComponent`). Side-by-side mapping: `.local/phases/_reference/FOODKING-CATALOG-MAP.md`.

---

## Dev commands

```bash
pnpm dev:portal                              # http://127.0.0.1:5175
pnpm seed:dev                                # categories + products (idempotent)
pnpm --filter @full-sales/portal lint test build
```

### Dev seed catalog (Phase 47)

After `pnpm seed:dev`, the portal catalog shows five categories with placeholder images on the first three:

| Slug | Name | Products (sample) |
|------|------|-------------------|
| `bebidas` | Bebidas | Refrigerante Cola 2L |
| `snacks` | Snacks | Batata Chips Original |
| `limpeza` | Limpeza | Detergente Neutro 500ml |
| `congelados` | Congelados | Pizza Congelada Mussarela (inactive) |
| `mercearia` | Mercearia | — (empty category for demo) |

Seed source: `backend/crates/dev-seed/src/catalog.rs` — upserts categories by slug on re-run.

---

## Catalog page flow

| Step | Behavior |
|------|----------|
| 1 | User opens `/` → redirects to `/?category=<first-active-slug>` |
| 2 | `CategoryBar` loads from `useCatalogCategories()` |
| 3 | Products load via `fetchPortalCategoryBySlug(slug)` |
| 4 | Client search filters within category (debounced) |
| 5 | List/grid toggle persisted in `localStorage` |
| 6 | Product detail at `/products/$id?category=<slug>` |

**Route files:** `routes/_authenticated/index.tsx`, `components/catalog/CatalogPageContent.tsx`

---

## Component library (`src/components/catalog/`)

| Component | Role |
|-----------|------|
| `CategoryBar` | Horizontal scroll category chips; arrow-key navigation |
| `ProductCardGrid` | Vertical product card |
| `ProductCardList` | Horizontal list row card |
| `ProductCatalog` | Composes bar + toolbar + cards |
| `CatalogToolbar` | Category title, search slot, list/grid toggle |
| `CatalogPageContent` | Catalog route body (data + composition) |
| `CatalogEmptyState` | Empty catalog with illustration |
| `CatalogSkeleton` | Loading placeholders |
| `ProductImage` | Shared image with `alt={product.name}` |

---

## Catalog utilities

| Path | Purpose |
|------|---------|
| `src/lib/catalog/catalogSearch.ts` | Search param parsing, slug validation, client filter |
| `src/lib/catalog/viewMode.ts` | `CatalogViewMode` + `localStorage` persistence |
| `src/lib/catalog/useCatalogCategories.ts` | React Query hook — `staleTime` 5 min |
| `src/lib/catalog/useDebouncedValue.ts` | Debounced search input |
| `src/lib/catalog/useCatalogRealtime.ts` | SSE invalidation for all catalog queries |

---

## API client

| Function | Endpoint (public / portal) |
|----------|----------------------------|
| `fetchPortalCategories` | `GET /v1/public/categories` or `/v1/portal/categories` |
| `fetchPortalCategoryBySlug` | `GET /v1/public/categories/{slug}` or `/v1/portal/categories/{slug}` |
| `fetchPortalProductById` | `GET /v1/portal/products/{id}` (public fallback when logged out) |
| `fetchPortalProducts` | `GET /v1/public/products?category=` (fallback) |

Types: `PortalCategory`, `PortalCategoryWithProducts`, `PortalProduct` in `src/lib/api/types.ts`.

---

## Portal shell

`PortalShell` prefetches categories and links **Catalog** to `/?category=<firstSlug>` (desktop nav + mobile bottom bar).

---

## i18n keys

`catalog.categories`, `catalog.selectCategory`, `catalog.emptyCategory`, `catalog.viewList`, `catalog.viewGrid`, `catalog.emptyDescription` (+ existing `catalog.*`).

---

## Testing

| Layer | Command |
|-------|---------|
| Unit + component | `pnpm --filter @full-sales/portal test` |

Key contracts: `catalogSearch.test.ts` (redirect + filter), Phase 45 component tests, `useCatalogRealtime.test.ts`.

Optional E2E: `pnpm test:e2e:portal` — `e2e/portal-catalog.spec.ts` (category URL, search, list/grid, add to cart).

---

## Known gaps

See `.local/phases/49-portal-product-detail-page/TASKS.md`:

- Portal product detail page still uses category-walk client lookup until Phase 49
- Product detail gallery UI (carousel) — Phase 49

---

**Updated:** 2026-07-05 (Phase 48 API)
