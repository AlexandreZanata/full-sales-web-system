import { expect, test } from '@playwright/test';

import { API_BASE, isApiReachable, login } from './helpers';

test.describe('Integration: portal order visible in admin', () => {
  test.skip(({ browserName }) => browserName !== 'chromium', 'chromium only');

  test('given_seeded_api_when_portal_submits_order_then_admin_lists_pending', async ({
    request,
  }) => {
    test.skip(!(await isApiReachable()), `API not reachable at ${API_BASE}`);

    const portalToken = await login('portal@seed-store.com', 'secret123');
    const adminToken = await login('admin@test.com', 'secret123');

    const productsResponse = await request.get(`${API_BASE}/v1/portal/products?pageSize=1`, {
      headers: { Authorization: `Bearer ${portalToken}` },
    });
    expect(productsResponse.ok()).toBeTruthy();
    const products = (await productsResponse.json()) as {
      items: Array<{ id: string }>;
    };
    const productId = products.items[0]?.id;
    expect(productId).toBeTruthy();

    const createResponse = await request.post(`${API_BASE}/v1/portal/orders`, {
      headers: {
        Authorization: `Bearer ${portalToken}`,
        'Content-Type': 'application/json',
      },
      data: {
        deliveryAddressId: '01900001-0011-7000-8000-000000000002',
        items: [{ productId, quantity: 1 }],
      },
    });
    expect(createResponse.status()).toBe(201);
    const order = (await createResponse.json()) as { id: string };
    const orderId = order.id;

    const submitResponse = await request.post(
      `${API_BASE}/v1/portal/orders/${orderId}/submit`,
      { headers: { Authorization: `Bearer ${portalToken}` } },
    );
    expect(submitResponse.ok()).toBeTruthy();

    const adminOrders = await request.get(
      `${API_BASE}/v1/orders?status=PendingApproval&pageSize=50`,
      { headers: { Authorization: `Bearer ${adminToken}` } },
    );
    expect(adminOrders.ok()).toBeTruthy();
    const list = (await adminOrders.json()) as { items: Array<{ id: string; status: string }> };
    const found = list.items.find((item) => item.id === orderId);
    expect(found?.status).toBe('PendingApproval');
  });
});
