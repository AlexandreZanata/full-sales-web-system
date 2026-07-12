import type { Page } from '@playwright/test';

const emptyCursorPage = {
  data: [],
  pagination: { next_cursor: null, has_more: false, limit: 20 },
};

/** Shell-level APIs that clear the session on 401 if left unmocked. */
export async function mockAdminShellApis(page: Page): Promise<void> {
  await page.route('**/v1/settings', async (route) => {
    if (route.request().method() !== 'GET') {
      await route.fallback();
      return;
    }
    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ displayName: 'Full Sales' }),
    });
  });

  await page.route('**/v1/billing/subscription', async (route) => {
    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({
        plan: {
          id: 'plan-starter',
          code: 'starter',
          name: 'Starter',
          priceMinor: 0,
          billingInterval: 'month',
        },
        status: 'active',
        tenantStatus: 'Active',
      }),
    });
  });
}

/** Dashboard widgets hit these on `/` after login. */
export async function mockAdminDashboardApis(page: Page): Promise<void> {
  await page.route('**/v1/orders?*', async (route) => {
    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({
        ...emptyCursorPage,
        pagination: { ...emptyCursorPage.pagination, limit: 1 },
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
      body: JSON.stringify({
        ...emptyCursorPage,
        pagination: { ...emptyCursorPage.pagination, limit: 5 },
      }),
    });
  });
}
