import type { Page, Route } from '@playwright/test';

import { buildPortalAccessToken, loginResponse } from './client-auth';

const EMPTY_PRODUCTS = { page: 1, pageSize: 50, total: 0, items: [] };

function isProductsList(path: string, method: string): boolean {
  return method === 'GET' && (path === '/portal/products' || path === '/public/products');
}

export async function mockPortalApi(page: Page): Promise<void> {
  const accessToken = buildPortalAccessToken();

  await page.route('**/v1/**', async (route: Route) => {
    const url = new URL(route.request().url());
    const path = url.pathname.replace(/^\/v1/, '');
    const method = route.request().method();

    if (path === '/auth/login' && method === 'POST') {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(loginResponse(accessToken)),
      });
      return;
    }

    if (isProductsList(path, method)) {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(EMPTY_PRODUCTS),
      });
      return;
    }

    if (path === '/portal/orders' && method === 'GET') {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ page: 1, pageSize: 20, total: 0, items: [] }),
      });
      return;
    }

    await route.fulfill({
      status: 404,
      contentType: 'application/json',
      body: JSON.stringify({ error: { code: 'NOT_FOUND', message: `Unmocked ${method} ${path}` } }),
    });
  });
}

export async function loginPortal(page: Page): Promise<void> {
  await page.goto('/login');
  await page.getByLabel('E-mail').fill('portal@seed-store.com');
  await page.getByLabel('Senha').fill('secret123');
  await page.getByRole('button', { name: 'Entrar', exact: true }).click();
}
