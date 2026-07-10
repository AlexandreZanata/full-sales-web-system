# Portal catalog (Phases 45–47, 71 FoodKing redesign)

> Guest + authenticated catalog in `apps/portal` (`@full-sales/portal`).  
> API: [API-CONTRACT.md](../API-CONTRACT.md) · Routes: [ROUTE-MATRIX.md](../ROUTE-MATRIX.md)

**Status:** Phase 71 complete (71A–71P) — PO visual sign-off pending (71Q).

**Design reference:** FoodKing UX — `.local/phases/71-portal-catalog-foodking-redesign/_reference/DESIGN-SPEC.md`

---

## Portal home demo (start here)

Full FoodKing-style home at `/` after seed + API:

```bash
# 1. Infrastructure + API (from repo root)
docker compose up -d          # Postgres, Redis, MinIO — if not already running
pnpm dev:api                  # http://127.0.0.1:8080

# 2. Seed catalog + portal home content (idempotent)
pnpm seed:dev                 # categories, products, banners, promotions, featured flags

# 3. Portal dev server
pnpm dev:portal               # http://127.0.0.1:5175

# 4. Verify
pnpm --filter @full-sales/portal lint test build
pnpm test:e2e:portal          # home sections, menu, cart, visual snapshots, axe
```

| URL | What you should see |
|-----|---------------------|
| `/` | Hero carousel → Our Menu chips → Featured grid → Offer cards → Popular list |
| `/?category=bebidas` | Menu catalog with category bar, search, grid/list toggle |
| `/products/{id}?category=slug` | Product detail gallery + sticky actions |

**Admin CMS** (optional content edits): `pnpm dev:admin` → `/portal` — hero banners, promotions, product `isFeatured`.

**Credentials:** dev seed admin `admin@seed-store.com` / `secret123` · portal guest needs no login.

---

## Home vs menu routing

| URL | Component | Sections / behavior |
|-----|-----------|---------------------|
| `/` | `CatalogHomePage` | Hero → Categories → Featured → Offers → Popular |
| `/?category={slug}` | `CatalogPageContent` | Category bar (menu variant), toolbar, product grid/list |
| `/?category={slug}&q={term}` | `CatalogPageContent` | Menu + debounced client search (header or toolbar) |
| `/products/{id}?category={slug}` | Product detail route | Gallery, specs, cart / order / WhatsApp CTAs |

**Routing file:** `routes/_authenticated/index.tsx` — renders home when `?category=` is absent; menu otherwise.

**Nav behavior:**

- Header **Home** → `/` (clears `category` / `q` via `catalogHomeSearch`)
- Header **Menu** / mobile bottom **Cardápio** → `/?category=<first active slug>`
- Header **Offers** → `/#offers` anchor on home promo section

---

## Home page components (`components/catalog/home/`)

| Component | `data-testid` | Hook / API | Role |
|-----------|---------------|------------|------|
| `CatalogHomePage` | `catalog-home-page` | — | Stacks all home sections; sr-only `<h1>` for a11y |
| `HeroBannerCarousel` | `hero-banner` | `useHeroBanners` → `GET /v1/public/banners?placement=hero` | Swiper autoplay; first slide `fetchPriority=high` + `sizes` for LCP |
| `HomeCategorySection` | `home-categories` | `useCatalogCategories` | "Our menu" title, View All pill, `CategoryBar` variant `home` |
| `FeaturedItemsSection` | `featured-items` | `useFeaturedProducts` → `GET /v1/public/products/featured` | 2–4 col grid, `ProductCardGrid` |
| `OfferBannersSection` | `offer-banners` | `usePromotions` → `GET /v1/public/promotions` | Pastel promo cards, `id="offers"` anchor |
| `PopularItemsSection` | `popular-items` | `usePopularProducts` → `GET /v1/public/products/popular` | 1–3 col list layout, `ProductCardList` |

**Fallbacks** (when API empty or unreachable): catalog list slice for featured/popular; demo promo cards; settings/demo hero SVG — see API client table below.

---

## Menu catalog components

| Component | Role |
|-----------|------|
| `CatalogPageContent` | Menu route body — `data-testid="catalog-menu"`, back-to-home link, URL `q` sync |
| `CategoryBar` | variant `menu`: active chip bottom border; variant `home`: scroll-snap chips |
| `CatalogToolbar` | Category title, search slot, list/grid toggle |
| `ProductCatalog` | Composes bar + toolbar + cards |
| `ProductCardGrid` / `ProductCardList` | FoodKing cards: image, description, price, Add pill |
| `ProductCardAddPill` | `.catalog-add-pill-btn` |
| `ProductInfoDialog` | Native `<dialog>` for extended description |
| `CatalogEmptyState` / `CatalogSkeleton` | Empty + loading states |

---

## Portal shell (`components/layout/`)

| Component | Role |
|-----------|------|
| `PortalShell` | Header + main + footer + mobile bottom nav + `CartFab` |
| `PortalHeader` | Logo, Home/Menu/Offers, desktop search pill, locale, cart pill, login |
| `PortalHeaderSearch` | Syncs header search with menu `?q=` and active category |
| `PortalFooter` | Red band: newsletter stub, links, contact phone, copyright |

---

## Product detail (`/products/$id`)

| Step | Behavior |
|------|----------|
| 1 | Open from catalog → `/products/{id}?category={slug}` |
| 2 | `fetchPortalProductById` — public when logged out |
| 3 | `ProductMediaPanel` + `ProductImageCarousel` when 2+ images |
| 4 | `ProductDetailActions` — cart, place order, WhatsApp (`salesContactPhone`) |

**Route:** `routes/_authenticated/products/$id.tsx`

---

## API client

| Function | Endpoint |
|----------|----------|
| `fetchPortalCategories` | `GET /v1/public/categories` (guest) or `/v1/portal/categories` |
| `fetchPortalCategoryBySlug` | `GET /v1/public/categories/{slug}` or portal variant |
| `fetchPortalProductById` | `GET /v1/public/products/{id}` or portal variant |
| `fetchPortalProducts` | `GET /v1/public/products` — catalog fallback |
| `fetchPortalBanners` | `GET /v1/public/banners?placement=hero` |
| `fetchPortalFeaturedProducts` | `GET /v1/public/products/featured` |
| `fetchPortalPromotions` | `GET /v1/public/promotions` |
| `fetchPortalPopularProducts` | `GET /v1/public/products/popular` |
| `fetchSettings` | `GET /v1/public/settings` or `/v1/settings` |

Guest calls use `skipAuth: true` on public paths. Types in `src/lib/api/types.ts`.

---

## Dev seed

### Catalog (`backend/crates/dev-seed/src/catalog.rs`)

| Slug | Name | Notes |
|------|------|-------|
| `bebidas` | Bebidas | Sample products |
| `snacks` | Snacks | Category thumbs on first three |
| `limpeza` | Limpeza | |
| `congelados` | Congelados | Includes inactive product demo |
| `mercearia` | Mercearia | Empty category demo |

### Portal home (`backend/crates/dev-seed/src/portal_content.rs`)

- 2 hero banners (`portal.banners`) with WebP in object storage
- 2 promotions (yellow + green) in `portal.promotions`
- First 3+ products marked `is_featured`
- Sales totals seeded for popular ranking

Re-run: `pnpm seed:dev` (idempotent upsert).

---

## Design tokens (Phase 71B)

| Token | Default | Runtime |
|-------|---------|---------|
| `--primary` | `#FE1F00` (oklch) | Overridden from `settings.themePrimaryColor` via `bootstrapPortalTheme()` |
| `--surface-muted` | `#F7F7FC` | Category chips |
| `--hairline` | `#EFF0F6` | Borders |
| Font | Rubik 400–700 | `index.html` Google Fonts |

Component classes: `.portal-header`, `.portal-footer`, `.catalog-add-pill-btn`, `.catalog-product-card-grid`, `.catalog-offer-card`.

---

## Admin CMS (Phase 71L)

| Path | Purpose |
|------|---------|
| `apps/admin` → `/portal` | CRUD hero banners + offer promotions |
| Product edit form | `isFeatured` checkbox → `PATCH /v1/products/{id}` |

RBAC: Admin only. Media entity types: `PortalBanner`, `PortalPromotion`.

---

## Testing

| Layer | Command |
|-------|---------|
| Unit + component | `pnpm --filter @full-sales/portal test` |
| Portal E2E | `pnpm test:e2e:portal` |
| Visual + a11y | `e2e/portal-home-visual.spec.ts` |
| Backend contracts | `cargo test -p api-http --test public_catalog` |

Key tests: `tests/catalog/*`, `CatalogPageContent.test.tsx`, home section tests, `portal-*-api.test.ts`, `public_catalog.rs`.

---

## Phase 71 acceptance (71Q)

| Check | Status | How verified |
|-------|--------|--------------|
| Home sections render in order | ✅ | `CatalogHomePage.test.tsx`, E2E home test |
| Menu category navigation + search + cart | ✅ | `portal-catalog.spec.ts`, `portal-order.spec.ts` |
| Mobile + desktop shell | ✅ | `portal-responsive.spec.ts` |
| Tenant primary color runtime | ✅ | `applyTheme.test.ts`, `bootstrapPortalTheme` on boot |
| Guest browse + login + checkout | ✅ | Portal E2E order flow |
| Hero LCP hints | ✅ | First slide `loading=eager`, `fetchPriority=high`, `sizes` |
| File size limits (≤200 lines) | ✅ | All `home/*` + `CatalogPageContent` under cap |
| Visual parity vs FoodKing | ⬜ | **PO review** — side-by-side with DESIGN-SPEC screenshots |
| WCAG AA brand contrast | ⬜ | Known gap: primary red on white (axe excludes in E2E) |

---

## Known gaps

| Gap | Description |
|-----|-------------|
| GAP-071-A11Y | Brand `#FE1F00` on white fails WCAG AA contrast — design debt |
| GAP-062 | Optional `/categories/$slug` path alias |
| GAP-049E | Related products rail on detail page |
| Newsletter | Footer submit stub until dedicated endpoint |

---

**Updated:** 2026-07-10 (Phase 71P documentation + 71Q automated acceptance)
