import { expect, test } from '@playwright/test';

import { buildPortalAccessToken, loginResponse } from './fixtures/client-auth';
import { seedPortalPtBrLocale } from './fixtures/locale';
import {
  MOCK_CATEGORY,
  MOCK_PRODUCT,
  handlePortalCatalogRoutes,
  fulfillPortalApiNotFound,
} from './fixtures/portal-catalog-mock';
import { loginPortal } from './fixtures/portal-api-mock';

test.describe('Portal order flow', () => {
  test('given_catalog_when_add_to_cart_and_submit_then_order_detail', async ({ page }) => {
    const productId = MOCK_PRODUCT.id;
    const orderId = '01900001-0030-7000-8000-000000000099';
    const accessToken = buildPortalAccessToken();

    await seedPortalPtBrLocale(page);
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

      if (path === '/portal/orders' && method === 'POST') {
        await route.fulfill({
          status: 201,
          contentType: 'application/json',
          body: JSON.stringify({
            id: orderId,
            status: 'Draft',
            deliveryAddressId: '01900001-0011-7000-8000-000000000002',
            totalAmount: 1500,
            totalCurrency: 'BRL',
            items: [
              {
                id: 'line-1',
                productId,
                quantity: 1,
                unitPriceAmount: 1500,
                unitPriceCurrency: 'BRL',
                lineTotalAmount: 1500,
              },
            ],
          }),
        });
        return;
      }

      if (path === `/portal/orders/${orderId}/submit` && method === 'POST') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            id: orderId,
            status: 'PendingApproval',
            deliveryAddressId: '01900001-0011-7000-8000-000000000002',
            totalAmount: 1500,
            totalCurrency: 'BRL',
            items: [
              {
                id: 'line-1',
                productId,
                quantity: 1,
                unitPriceAmount: 1500,
                unitPriceCurrency: 'BRL',
                lineTotalAmount: 1500,
              },
            ],
          }),
        });
        return;
      }

      if (path === `/portal/orders/${orderId}` && method === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            id: orderId,
            status: 'PendingApproval',
            deliveryAddressId: '01900001-0011-7000-8000-000000000002',
            totalAmount: 1500,
            totalCurrency: 'BRL',
            items: [
              {
                id: 'line-1',
                productId,
                quantity: 1,
                unitPriceAmount: 1500,
                unitPriceCurrency: 'BRL',
                lineTotalAmount: 1500,
              },
            ],
          }),
        });
        return;
      }

      await fulfillPortalApiNotFound(route, method, path);
    });

    await loginPortal(page);

    await page.getByLabel('Main').getByRole('link', { name: 'Cardápio', exact: true }).click();
    await expect(page).toHaveURL(new RegExp(`category=${MOCK_CATEGORY.slug}`));
    await expect(page.getByTestId('catalog-menu')).toBeVisible();
    await page.getByTestId('catalog-menu').getByRole('button', { name: 'Adicionar ao carrinho' }).first().click();
    await page.locator('a.portal-cart-pill').filter({ visible: true }).click();

    await expect(page).toHaveURL('/cart');
    await expect(page.getByText(MOCK_PRODUCT.name)).toBeVisible();
    await page.getByRole('button', { name: 'Enviar pedido' }).click();

    await expect(page).toHaveURL(new RegExp(`/orders/${orderId}`));
    await expect(page.getByText('Aguardando aprovação').first()).toBeVisible();
  });
});
