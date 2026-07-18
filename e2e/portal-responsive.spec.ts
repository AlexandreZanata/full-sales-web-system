import { expect, test } from '@playwright/test';

import { mockPortalApi } from './fixtures/portal-api-mock';
import { MOCK_CATEGORY } from './fixtures/portal-catalog-mock';

const VIEWPORTS = [
  { label: '390px mobile', width: 390, height: 844, expectMobileNav: true },
  { label: '1024px tablet', width: 1024, height: 1024, expectMobileNav: false },
  { label: '1280px desktop', width: 1280, height: 800, expectMobileNav: false },
] as const;

test.describe('Portal responsive shell', () => {
  for (const viewport of VIEWPORTS) {
    test(`given_${viewport.label}_when_guest_then_nav_matches_breakpoint`, async ({ page }) => {
      await page.setViewportSize({ width: viewport.width, height: viewport.height });
      await mockPortalApi(page);
      await page.goto('/');
      await expect(page.getByTestId('catalog-home-page')).toBeVisible();

      const mobileNav = page.getByRole('navigation', { name: 'Mobile' });
      const desktopNav = page.getByRole('navigation', { name: 'Main' });

      if (await desktopNav.isVisible()) {
        await desktopNav.getByRole('link', { name: 'Cardápio', exact: true }).click();
      } else {
        await mobileNav.getByRole('link', { name: 'Cardápio', exact: true }).click();
      }
      await expect(page).toHaveURL(new RegExp(`category=${MOCK_CATEGORY.slug}`));

      if (viewport.expectMobileNav) {
        await expect(mobileNav).toBeVisible();
        await expect(desktopNav).toBeHidden();
        await expect(mobileNav.getByRole('link', { name: 'Conta', exact: true })).toBeVisible();
        await expect(mobileNav.getByRole('link', { name: 'Carrinho', exact: true })).toBeVisible();
      } else {
        await expect(mobileNav).toBeHidden();
        await expect(desktopNav).toBeVisible();
      }

      await expect(page.getByTestId('catalog-menu')).toBeVisible();
    });
  }
});
