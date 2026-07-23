import type { Page, Route } from '@playwright/test';

import { buildPortalAccessToken, loginResponse } from './client-auth';
import { seedPortalPtBrLocale } from './locale';
import { handlePortalCatalogRoutes, fulfillPortalApiNotFound } from './portal-catalog-mock';

export async function mockPortalApi(page: Page): Promise<void> {
  await seedPortalPtBrLocale(page);
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

    if (await handlePortalCatalogRoutes(route)) {
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

    await fulfillPortalApiNotFound(route, method, path);
  });
}

export async function loginPortal(page: Page): Promise<void> {
  await seedPortalPtBrLocale(page);
  await page.goto('/login');
  await page.getByLabel('E-mail').waitFor({ state: 'visible' });
  await page.getByLabel('E-mail').fill('portal@seed-store.com');
  await page.getByLabel('Senha').fill('secret123');
  await page.getByRole('button', { name: 'Entrar', exact: true }).click();
}
