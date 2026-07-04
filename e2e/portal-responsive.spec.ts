import { expect, test } from '@playwright/test';

import { loginPortal, mockPortalApi } from './fixtures/portal-api-mock';

const VIEWPORTS = [
  { label: '390px mobile', width: 390, height: 844, expectMobileNav: true },
  { label: '768px tablet', width: 768, height: 1024, expectMobileNav: false },
  { label: '1280px desktop', width: 1280, height: 800, expectMobileNav: false },
] as const;

test.describe('Portal responsive shell', () => {
  for (const viewport of VIEWPORTS) {
    test(`given_${viewport.label}_when_authenticated_then_nav_matches_breakpoint`, async ({
      page,
    }) => {
      await page.setViewportSize({ width: viewport.width, height: viewport.height });
      await mockPortalApi(page);
      await loginPortal(page);
      await expect(page).toHaveURL('/');

      const mobileNav = page.getByRole('navigation', { name: 'Mobile' });
      const desktopNav = page.getByRole('navigation', { name: 'Main' });

      if (viewport.expectMobileNav) {
        await expect(mobileNav).toBeVisible();
        await expect(desktopNav).toBeHidden();
      } else {
        await expect(mobileNav).toBeHidden();
        await expect(desktopNav).toBeVisible();
      }

      await expect(page.getByRole('heading', { name: 'Catálogo' })).toBeVisible();
    });
  }
});
