import { expect, test } from '@playwright/test';

import { mockAdminShellApis } from './fixtures/admin-shell-api';
import { loginResponse } from './fixtures/auth';
import { seedEnglishLocale } from './fixtures/locale';
import { openNavLink } from './fixtures/nav';

const seededOrders = {
  data: [
    {
      id: '770e8400-e29b-41d4-a716-446655440000',
      status: 'PendingApproval',
      commerceId: '880e8400-e29b-41d4-a716-446655440001',
      totalAmount: 1500,
      totalCurrency: 'BRL',
      createdAt: '2026-07-01T12:00:00Z',
    },
  ],
  pagination: { next_cursor: null, has_more: false, limit: 20 },
};

const emptyCursorPage = {
  data: [],
  pagination: { next_cursor: null, has_more: false, limit: 20 },
};

test.describe('Admin orders list', () => {
  test.beforeEach(async ({ page }) => {
    await seedEnglishLocale(page);
    await mockAdminShellApis(page);
    await page.route('**/v1/auth/login', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(loginResponse()),
      });
    });
    await page.route('**/v1/orders?*', async (route) => {
      const url = route.request().url();
      if (url.includes('filter%5Bstatus%5D=PendingApproval')) {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ ...emptyCursorPage, pagination: { ...emptyCursorPage.pagination, limit: 1 } }),
        });
        return;
      }
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(seededOrders),
      });
    });
    await page.route('**/v1/commerces?**', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          data: [
            {
              id: '880e8400-e29b-41d4-a716-446655440001',
              cnpj: '11222333000181',
              legalName: 'Seed Commerce LTDA',
              tradeName: 'Seed Store',
              active: true,
            },
          ],
          pagination: { next_cursor: null, has_more: false, limit: 50 },
        }),
      });
    });
    await page.route('**/v1/deliveries?*', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(emptyCursorPage),
      });
    });
    await page.route('**/v1/sales?*', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ ...emptyCursorPage, pagination: { ...emptyCursorPage.pagination, limit: 5 } }),
      });
    });

    await page.goto('/login');
    await page.getByLabel('Email').fill('admin@test.com');
    await page.getByLabel('Password').fill('secret123');
    await page.getByRole('button', { name: 'Sign in', exact: true }).click();
    await expect(page).toHaveURL('/');
  });

  test('given_seeded_orders_when_open_list_then_shows_order_row', async ({ page }) => {
    await openNavLink(page, 'Orders');
    await expect(page).toHaveURL('/orders');
    await expect(page.getByRole('heading', { name: 'Orders' })).toBeVisible();
    await expect(page.getByRole('cell', { name: 'Pending approval' })).toBeVisible();
  });
});
