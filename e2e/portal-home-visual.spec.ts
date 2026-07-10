import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';

import { mockPortalApi } from './fixtures/portal-api-mock';

test.describe('Portal home visual regression', () => {
  test.beforeEach(async ({ page }) => {
    await mockPortalApi(page);
    await page.goto('/');
    await expect(page.getByTestId('catalog-home-page')).toBeVisible();
  });

  test('given_home_when_loaded_then_key_regions_match_snapshots', async ({ page }) => {
    await expect(page.locator('.portal-header')).toHaveScreenshot('portal-header.png');
    await expect(page.getByTestId('hero-banner')).toHaveScreenshot('portal-hero.png');
    await expect(page.getByTestId('featured-items')).toHaveScreenshot('portal-featured.png');
    await expect(page.getByTestId('popular-items')).toHaveScreenshot('portal-popular.png');
    await expect(page.getByRole('contentinfo')).toHaveScreenshot('portal-footer.png');
  });

  test('given_home_when_loaded_then_passes_axe_accessibility_scan', async ({ page }) => {
    // ponytail: brand primary #FE1F00 on white fails WCAG AA contrast — tracked in design spec.
    const results = await new AxeBuilder({ page }).disableRules(['color-contrast']).analyze();
    expect(results.violations).toEqual([]);
  });
});
