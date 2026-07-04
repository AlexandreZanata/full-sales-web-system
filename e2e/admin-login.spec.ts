import { expect, test } from '@playwright/test';

import { loginResponse } from './fixtures/auth';
import { seedEnglishLocale } from './fixtures/locale';

test.describe('Admin login', () => {
  test('given_valid_credentials_when_sign_in_then_dashboard', async ({ page }) => {
    await seedEnglishLocale(page);
    await page.route('**/v1/auth/login', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify(loginResponse('admin@test.com')),
      });
    });

    await page.goto('/login');
    await page.getByLabel('Email').fill('admin@test.com');
    await page.getByLabel('Password').fill('secret123');
    await page.getByRole('button', { name: 'Sign in', exact: true }).click();

    await expect(page).toHaveURL('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });
});
