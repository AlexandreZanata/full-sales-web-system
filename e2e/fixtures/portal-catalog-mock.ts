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
  return method === 'GET' && /^\/(portal|public)\/products\/[^/]+$/.test(path);
}

export function isCatalogEvents(path: string, method: string): boolean {
  return method === 'GET' && path === '/public/catalog/events';
}

export async function fulfillCategoriesList(route: Route): Promise<void> {
  await route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify(MOCK_CATEGORIES),
  });
}

export async function fulfillCategoryBySlug(route: Route): Promise<void> {
  await route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify({
      ...MOCK_CATEGORY,
      products: [MOCK_PRODUCT],
      page: 1,
      pageSize: 50,
      total: 1,
    }),
  });
}

export async function fulfillProductsList(route: Route): Promise<void> {
  await route.fulfill({
    status: 200,
    contentType: 'application/json',
    body: JSON.stringify({
      page: 1,
      pageSize: 50,
      total: 1,
      items: [MOCK_PRODUCT],
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

  if (isProductById(path, method)) {
    await fulfillProductById(route);
    return true;
  }

  if (isCatalogEvents(path, method)) {
    await fulfillCatalogEvents(route);
    return true;
  }

  return false;
}
