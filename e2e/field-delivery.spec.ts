import { expect, test } from '@playwright/test';

import { buildFieldAccessToken, loginResponse } from './fixtures/client-auth';
import { cursorPage } from './fixtures/cursor-page';

test.describe('Field delivery flow', () => {
  test('given_driver_when_list_and_start_transit_then_status_updates', async ({ page }) => {
    const deliveryId = '01900001-0040-7000-8000-000000000001';
    const orderId = '01900001-0030-7000-8000-000000000001';
    const accessToken = buildFieldAccessToken('driver-a@test.com', 'Driver');
    let inTransit = false;

    await page.route('**/v1/**', async (route) => {
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

      if (path === '/deliveries' && method === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify(
            cursorPage([
              {
                id: deliveryId,
                orderId,
                driverId: '01900001-0002-7000-8000-000000000001',
                status: 'Waiting',
              },
            ]),
          ),
        });
        return;
      }

      if (path === '/sales' && method === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify(cursorPage([])),
        });
        return;
      }

      if (path === `/deliveries/${deliveryId}` && method === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            id: deliveryId,
            orderId,
            driverId: '01900001-0002-7000-8000-000000000001',
            status: inTransit ? 'InTransit' : 'Waiting',
            orderItems: [
              {
                id: '01900001-0031-7000-8000-000000000001',
                productId: '01900001-0020-7000-8000-000000000001',
                quantity: 1,
              },
            ],
          }),
        });
        return;
      }

      if (path === `/deliveries/${deliveryId}/start-transit` && method === 'POST') {
        inTransit = true;
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            id: deliveryId,
            orderId,
            driverId: '01900001-0002-7000-8000-000000000001',
            status: 'InTransit',
            orderItems: [
              {
                id: '01900001-0031-7000-8000-000000000001',
                productId: '01900001-0020-7000-8000-000000000001',
                quantity: 1,
              },
            ],
          }),
        });
        return;
      }

      await route.fulfill({
        status: 404,
        contentType: 'application/json',
        body: JSON.stringify({ error: { code: 'NOT_FOUND', message: `Unmocked ${method} ${path}` } }),
      });
    });

    await page.goto('/login');
    await page.getByLabel('E-mail').fill('driver-a@test.com');
    await page.getByLabel('Senha').fill('secret123');
    await page.getByRole('button', { name: 'Entrar', exact: true }).click();

    await page.getByRole('link', { name: 'Entregas' }).first().click();
    await expect(page).toHaveURL('/deliveries');
    await page.getByRole('link').filter({ hasText: 'Pedido' }).first().click();
    await page.getByRole('button', { name: 'Iniciar trânsito' }).click();
    await expect(page.getByText('Em trânsito').first()).toBeVisible();
  });
});
