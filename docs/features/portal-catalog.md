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
| `ProductCardList` | Horizontal list row card — larger mobile thumbnail, grouped SKU/category meta, prominent price, full-width CTA on small screens |
| `ProductCatalog` | Composes bar + toolbar + cards |
| `CatalogToolbar` | Category title, search slot, list/grid toggle |
| `CatalogPageContent` | Catalog route body (data + composition) |
| `CatalogEmptyState` | Empty catalog with illustration |
| `CatalogSkeleton` | Loading placeholders |
| `ProductImage` | Shared image with `alt={product.name}` |
| `ProductImageCarousel` | Loop carousel with prev/next and dot indicators |
| `ProductMediaPanel` | Sticky gallery wrapper for product detail |
| `ProductDetailInfo` | Title, price, description, specs table |
| `ProductDetailSkeleton` | Product detail loading state |
| `ProductDetailActions` | Sticky CTAs: cart, place order, contact seller |

---

## Product detail page (`/products/$id`)

| Step | Behavior |
|------|----------|
| 1 | User opens product from catalog → `/products/{id}?category={slug}` |
| 2 | `fetchPortalProductById(id)` → `GET /v1/portal/products/{id}` (public when logged out) |
| 3 | `ProductMediaPanel` builds gallery from `primaryImageUrl` + `imageUrls[]` |
| 4 | Single image: no carousel chrome; 2+ images: arrows + dots |
| 5 | Specs show SKU, unit of measure, category, status |
| 6 | `ProductDetailActions` sticky bar: add to cart, place order (→ cart), contact seller (WhatsApp) |
| 7 | Contact seller uses `salesContactPhone` from `GET /v1/public/settings` (guest) or `/v1/settings` (logged in) |

**Route file:** `routes/_authenticated/products/$id.tsx`

**Components:** `ProductDetailActions`, `lib/contact/sellerWhatsAppLink.ts`

---

## Catalog utilities

| Path | Purpose |
|------|---------|
| `src/lib/catalog/gallerySlides.ts` | `buildGallerySlides` — primary first, dedupe |
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
| `fetchPortalProductById` | `GET /v1/public/products/{id}` or `/v1/portal/products/{id}` |
| `fetchSettings` | `GET /v1/public/settings` or `/v1/settings` |
| `fetchPortalProducts` | `GET /v1/public/products?category=` (fallback) |

Types: `PortalCategory`, `PortalCategoryWithProducts`, `PortalProduct`, `PortalProductDetail` in `src/lib/api/types.ts`.

---

## Portal shell

`PortalShell` prefetches categories and links **Catalog** to `/?category=<firstSlug>` (desktop nav + mobile bottom bar). Catalog nav stays active on `/products/$id` routes (GAP-061).

Header uses `.portal-header` (Phase 71B). Full FoodKing-style nav (search pill, cart total, login pill) lands in Phase 71C.

---

## Design tokens (Phase 71B)

| Path | Purpose |
|------|---------|
| `src/styles/theme.css` | FoodKing-aligned CSS variables and component classes |
| `src/lib/settings/applyTheme.ts` | `hexToOklch`, `applyThemePrimaryColor`, `bootstrapPortalTheme` |

| Token | Default | Notes |
|-------|---------|-------|
| `--primary` | `#FE1F00` (oklch) | Overridden at boot from `settings.themePrimaryColor` when present |
| `--surface-muted` | `#F7F7FC` | Category chip background |
| `--hairline` | `#EFF0F6` | Borders, search pill |
| Font | Rubik 400–700 | Google Fonts in `index.html` |

Component classes: `.portal-header`, `.portal-footer`, `.catalog-add-pill-btn`, `.catalog-product-card-grid` (`rounded-2xl`).

Spec appendix: `.local/phases/71-portal-catalog-foodking-redesign/_reference/DESIGN-SPEC.md`.

---

## i18n keys

`catalog.categories`, `catalog.selectCategory`, `catalog.emptyCategory`, `catalog.viewList`, `catalog.viewGrid`, `catalog.emptyDescription`, `productDetail.*` (+ existing `catalog.*`).

---

## Testing

| Layer | Command |
|-------|---------|
| Unit + component | `pnpm --filter @full-sales/portal test` |

Key contracts: `catalogSearch.test.ts` (redirect + filter), Phase 45 component tests, `ProductCardList.test.tsx` (list card hierarchy + metadata), `useCatalogRealtime.test.ts`, `gallerySlides.test.ts`, `portal-product-detail-api.test.ts`, `applyTheme.test.ts` (brand color runtime).

Optional E2E: `pnpm test:e2e:portal` — `e2e/portal-catalog.spec.ts` (category URL, search, list/grid, add to cart, product detail carousel).

---

## Known gaps

| Gap | Description | Owner |
|-----|-------------|-------|
| GAP-062 | Optional `/categories/$slug` path alias | Deferred |
| GAP-049E | Related products rail on detail page | Phase 49 optional / future |
| Phase 50 | Enhanced sticky bar actions (quantity stepper) | Phase 50 |

---

**Updated:** 2026-07-10 (Phase 71B design tokens)
