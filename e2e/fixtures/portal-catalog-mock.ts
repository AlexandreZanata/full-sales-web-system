import type { Route } from '@playwright/test';

export const MOCK_CATEGORY = {
  id: '01900001-0010-7000-8000-000000000010',
  name: 'Snacks',
  slug: 'snacks',
  sortOrder: 1,
  active: true,
};

export const MOCK_PRODUCT = {
  id: '01900001-0020-7000-8000-000000000001',
  name: 'Seed Widget',
  sku: 'SKU-001',
  priceAmount: 1500,
  priceCurrency: 'BRL',
  categoryId: MOCK_CATEGORY.id,
  categoryName: MOCK_CATEGORY.name,
  categorySlug: MOCK_CATEGORY.slug,
  description: 'Crispy seeded snack for portal E2E.',
};

export const MOCK_FEATURED_PRODUCT = {
  ...MOCK_PRODUCT,
  id: '01900001-0020-7000-8000-000000000002',
  name: 'Featured Burger',
  sku: 'FEAT-001',
  description: 'Chef special featured item.',
};

export const MOCK_POPULAR_PRODUCT = {
  ...MOCK_PRODUCT,
  id: '01900001-0020-7000-8000-000000000003',
  name: 'Popular Soda',
  sku: 'POP-001',
  description: 'Best-selling cold drink.',
};

export const MOCK_BANNER = {
  id: '01900001-0030-7000-8000-000000000001',
  imageUrl: 'https://cdn.example/hero-banner.jpg',
  altText: 'Welcome to our menu',
};

export const MOCK_PROMOTION = {
  id: '01900001-0030-7000-8000-000000000002',
  headline: 'Tasty Burger',
  discountText: '30% OFF',
  background: 'yellow',
  categorySlug: MOCK_CATEGORY.slug,
};

export const MOCK_PRODUCT_DETAIL = {
  ...MOCK_PRODUCT,
  unitOfMeasure: 'UN',
  primaryImageUrl: 'https://cdn.example/primary.jpg',
  imageUrls: ['https://cdn.example/gallery-1.jpg'],
  description: 'Seed product for E2E.',
};

export const MOCK_CATEGORIES = [MOCK_CATEGORY];

export function isCategoriesList(path: string, method: string): boolean {
  return method === 'GET' && (path === '/portal/categories' || path === '/public/categories');
}

export function isCategoryBySlug(path: string, method: string): boolean {
  return method === 'GET' && /^\/(portal|public)\/categories\/[^/]+$/.test(path);
}

export function isProductsList(path: string, method: string): boolean {
  return method === 'GET' && (path === '/portal/products' || path === '/public/products');
}

export function isProductById(path: string, method: string): boolean {
  return (
    method === 'GET' &&
    /^\/(portal|public)\/products\/[^/]+$/.test(path) &&
    !path.endsWith('/featured') &&
    !path.endsWith('/popular')
  );
}

export function isCatalogEvents(path: string, method: string): boolean {
  return method === 'GET' && path === '/public/catalog/events';
}

export async function fulfillCategoriesList(route: Route): Promise<void> {
  await route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify({
      data: MOCK_CATEGORIES,
      pagination: { next_cursor: null, has_more: false, limit: 100 },
    }),
  });
}

export async function fulfillCategoryBySlug(route: Route): Promise<void> {
  await route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify({
      ...MOCK_CATEGORY,
      products: [MOCK_PRODUCT],
      pagination: { next_cursor: null, has_more: false, limit: 50 },
    }),
  });
}

export async function fulfillProductsList(route: Route): Promise<void> {
  await route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify({
      data: [MOCK_PRODUCT],
      pagination: { next_cursor: null, has_more: false, limit: 50 },
    }),
  });
}

export async function fulfillProductById(route: Route): Promise<void> {
  await route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify(MOCK_PRODUCT_DETAIL),
  });
}

export async function fulfillCatalogEvents(route: Route): Promise<void> {
  await route.fulfill({ status: 200, contentType: 'text/event-stream', body: '' });
}

export async function fulfillPublicSettings(route: Route): Promise<void> {
  await route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify({
      displayName: 'Dev Sales Platform',
      salesContactPhone: '5511987654321',
    }),
  });
}

export function isPublicSettings(path: string, method: string): boolean {
  return method === 'GET' && path === '/public/settings';
}

export function isPublicBanners(path: string, method: string): boolean {
  return method === 'GET' && path.startsWith('/public/banners');
}

export function isPublicPromotions(path: string, method: string): boolean {
  return method === 'GET' && path.startsWith('/public/promotions');
}

export function isPublicFeaturedProducts(path: string, method: string): boolean {
  return method === 'GET' && path.startsWith('/public/products/featured');
}

export function isPublicPopularProducts(path: string, method: string): boolean {
  return method === 'GET' && path.startsWith('/public/products/popular');
}

export async function fulfillPublicBanners(route: Route): Promise<void> {
  await route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify({
      data: [MOCK_BANNER],
      pagination: { next_cursor: null, has_more: false, limit: 5 },
    }),
  });
}

export async function fulfillPublicPromotions(route: Route): Promise<void> {
  await route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify({
      data: [MOCK_PROMOTION],
      pagination: { next_cursor: null, has_more: false, limit: 4 },
    }),
  });
}

export async function fulfillPublicFeaturedProducts(route: Route): Promise<void> {
  await route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify({
      data: [MOCK_FEATURED_PRODUCT],
      pagination: { next_cursor: null, has_more: false, limit: 12 },
    }),
  });
}

export async function fulfillPublicPopularProducts(route: Route): Promise<void> {
  await route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify({
      data: [MOCK_POPULAR_PRODUCT],
      pagination: { next_cursor: null, has_more: false, limit: 12 },
    }),
  });
}

export async function fulfillPortalApiNotFound(route: Route, method: string, path: string): Promise<void> {
  await route.fulfill({
    status: 404,
    contentType: 'application/json',
    body: JSON.stringify({ error: { code: 'NOT_FOUND', message: `Unmocked ${method} ${path}` } }),
  });
}

export async function handlePortalCatalogRoutes(route: Route): Promise<boolean> {
  const url = new URL(route.request().url());
  const path = url.pathname.replace(/^\/v1/, '');
  const method = route.request().method();

  if (isCategoriesList(path, method)) {
    await fulfillCategoriesList(route);
    return true;
  }

  if (isCategoryBySlug(path, method)) {
    await fulfillCategoryBySlug(route);
    return true;
  }

  if (isProductsList(path, method)) {
    await fulfillProductsList(route);
    return true;
  }

  if (isPublicFeaturedProducts(path, method)) {
    await fulfillPublicFeaturedProducts(route);
    return true;
  }

  if (isPublicPopularProducts(path, method)) {
    await fulfillPublicPopularProducts(route);
    return true;
  }

  if (isProductById(path, method)) {
    await fulfillProductById(route);
    return true;
  }

  if (isCatalogEvents(path, method)) {
    await fulfillCatalogEvents(route);
    return true;
  }

  if (isPublicSettings(path, method)) {
    await fulfillPublicSettings(route);
    return true;
  }

  if (isPublicBanners(path, method)) {
    await fulfillPublicBanners(route);
    return true;
  }

  if (isPublicPromotions(path, method)) {
    await fulfillPublicPromotions(route);
    return true;
  }

  return false;
}
