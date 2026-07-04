import type { Page, Route } from '@playwright/test';

import { buildFieldAccessToken, loginResponse } from './client-auth';

export async function mockFieldApi(page: Page): Promise<void> {
  const accessToken = buildFieldAccessToken();

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

    if (path === '/sales' && method === 'GET') {
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

export async function loginField(page: Page): Promise<void> {
  await page.goto('/login');
  await page.getByLabel('E-mail').fill('seller@test.com');
  await page.getByLabel('Senha').fill('secret123');
  await page.getByRole('button', { name: 'Entrar', exact: true }).click();
}
