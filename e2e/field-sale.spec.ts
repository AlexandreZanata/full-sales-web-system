import { expect, test } from '@playwright/test';

import { buildFieldAccessToken, loginResponse } from './fixtures/client-auth';
import { cursorPage } from './fixtures/cursor-page';

test.describe('Field sale flow', () => {
  test('given_new_sale_when_confirm_then_detail_shows_confirmed', async ({ page }) => {
    const saleId = '01900001-0050-7000-8000-000000000099';
    const commerceId = '01900001-0010-7000-8000-000000000001';
    const productId = '01900001-0020-7000-8000-000000000001';
    const accessToken = buildFieldAccessToken();

    let saleConfirmed = false;

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

      if (path === '/commerces' && method === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify(
            cursorPage(
              [{ id: commerceId, legalName: 'Seed Store', tradeName: 'Seed Store', active: true }],
              100,
            ),
          ),
        });
        return;
      }

      if (path === '/products' && method === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify(
            cursorPage(
              [
                {
                  id: productId,
                  name: 'Seed Widget',
                  sku: 'SKU-001',
                  priceAmount: 2500,
                  priceCurrency: 'BRL',
                  active: true,
                },
              ],
              100,
            ),
          ),
        });
        return;
      }

      if (path === `/inventory/products/${productId}/balance` && method === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ productId, available: 100, asOf: new Date().toISOString() }),
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

      if (path === '/sales' && method === 'POST') {
        await route.fulfill({
          status: 201,
          contentType: 'application/json',
          body: JSON.stringify({
            id: saleId,
            commerceId,
            driverId: '01900001-0003-7000-8000-000000000001',
            status: 'Pending',
            paymentMethod: 'pix',
            totalAmount: 2500,
            totalCurrency: 'BRL',
            items: [
              {
                productId,
                quantity: 1,
                unitPriceAmount: 2500,
                unitPriceCurrency: 'BRL',
                lineTotalAmount: 2500,
              },
            ],
          }),
        });
        return;
      }

      if (path === `/sales/${saleId}/confirm` && method === 'POST') {
        saleConfirmed = true;
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            id: saleId,
            commerceId,
            driverId: '01900001-0003-7000-8000-000000000001',
            status: 'Confirmed',
            paymentMethod: 'pix',
            totalAmount: 2500,
            totalCurrency: 'BRL',
            items: [
              {
                productId,
                quantity: 1,
                unitPriceAmount: 2500,
                unitPriceCurrency: 'BRL',
                lineTotalAmount: 2500,
              },
            ],
          }),
        });
        return;
      }

      if (path === `/sales/${saleId}` && method === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            id: saleId,
            commerceId,
            driverId: '01900001-0003-7000-8000-000000000001',
            status: saleConfirmed ? 'Confirmed' : 'Pending',
            paymentMethod: 'pix',
            totalAmount: 2500,
            totalCurrency: 'BRL',
            items: [
              {
                productId,
                quantity: 1,
                unitPriceAmount: 2500,
                unitPriceCurrency: 'BRL',
                lineTotalAmount: 2500,
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
    await page.getByLabel('E-mail').fill('seller@test.com');
    await page.getByLabel('Senha').fill('secret123');
    await page.getByRole('button', { name: 'Entrar', exact: true }).click();
    await expect(page).toHaveURL('/');

    await page.getByRole('link', { name: 'Nova venda' }).first().click();
    await page.getByLabel('Comércio').selectOption(commerceId);
    await page.getByLabel('Pagamento').selectOption('pix');
    await page.getByLabel('Produto').selectOption(productId);
    await page.getByRole('button', { name: 'Confirmar' }).click();

    await expect(page).toHaveURL(new RegExp(`/sales/${saleId}`));
    await page.getByRole('button', { name: 'Confirmar venda' }).click();
    await expect(page.getByText('Confirmada').first()).toBeVisible();
  });
});
