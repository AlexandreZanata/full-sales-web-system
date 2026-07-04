import { expect, test } from '@playwright/test';

import { API_BASE, isApiReachable, login } from './helpers';

test.describe('Integration: field sale visible in admin', () => {
  test.skip(({ browserName }) => browserName !== 'chromium', 'chromium only');

  test('given_seeded_api_when_seller_confirms_sale_then_admin_lists_confirmed', async ({
    request,
  }) => {
    test.skip(!(await isApiReachable()), `API not reachable at ${API_BASE}`);

    const sellerToken = await login('driver-a@test.com', 'secret123');
    const adminToken = await login('admin@test.com', 'secret123');

    const commercesResponse = await request.get(`${API_BASE}/v1/commerces?pageSize=1`, {
      headers: { Authorization: `Bearer ${sellerToken}` },
    });
    expect(commercesResponse.ok()).toBeTruthy();
    const commerces = (await commercesResponse.json()) as {
      items: Array<{ id: string }>;
    };
    const commerceId = commerces.items[0]?.id;
    expect(commerceId).toBeTruthy();

    const productsResponse = await request.get(`${API_BASE}/v1/products?pageSize=1`, {
      headers: { Authorization: `Bearer ${sellerToken}` },
    });
    expect(productsResponse.ok()).toBeTruthy();
    const products = (await productsResponse.json()) as {
      items: Array<{ id: string }>;
    };
    const productId = products.items[0]?.id;
    expect(productId).toBeTruthy();

    const idempotencyKey = `integration-${Date.now()}`;
    const createResponse = await request.post(`${API_BASE}/v1/sales`, {
      headers: {
        Authorization: `Bearer ${sellerToken}`,
        'Content-Type': 'application/json',
        'Idempotency-Key': idempotencyKey,
      },
      data: {
        commerceId,
        paymentMethod: 'pix',
        items: [{ productId, quantity: 1 }],
      },
    });
    expect(createResponse.status()).toBe(201);
    const sale = (await createResponse.json()) as { id: string };
    const saleId = sale.id;

    const confirmResponse = await request.post(`${API_BASE}/v1/sales/${saleId}/confirm`, {
      headers: { Authorization: `Bearer ${sellerToken}` },
    });
    expect(confirmResponse.ok()).toBeTruthy();

    const adminSales = await request.get(`${API_BASE}/v1/sales?status=Confirmed&pageSize=50`, {
      headers: { Authorization: `Bearer ${adminToken}` },
    });
    expect(adminSales.ok()).toBeTruthy();
    const list = (await adminSales.json()) as { items: Array<{ id: string; status: string }> };
    const found = list.items.find((item) => item.id === saleId);
    expect(found?.status).toBe('Confirmed');
  });
});
