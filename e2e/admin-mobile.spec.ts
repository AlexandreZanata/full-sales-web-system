import { expect, test } from '@playwright/test';

import { seedEnglishLocale } from './fixtures/locale';

test.describe('Admin mobile navigation', () => {
  test.use({ viewport: { width: 390, height: 844 } });

  test('given_mobile_viewport_when_open_menu_then_sidebar_links_visible', async ({ page }) => {
    await seedEnglishLocale(page);
    await page.goto('/login');
    await page.getByRole('button', { name: 'Enter admin shell (dev)' }).click();
    await expect(page).toHaveURL('/');

    await page.getByRole('button', { name: 'Open navigation menu' }).click();
    await expect(page.getByRole('dialog', { name: 'Navigation menu' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Orders' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Reports' })).toBeVisible();

    await page.getByRole('link', { name: 'Audit' }).click();
    await expect(page).toHaveURL('/audit');
    await expect(page.getByRole('heading', { name: 'Audit log' })).toBeVisible();
  });
});
