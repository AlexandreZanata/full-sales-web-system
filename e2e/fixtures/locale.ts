import type { Page } from '@playwright/test';

/** Force English locale so admin role-based selectors stay stable in E2E. */
export async function seedEnglishLocale(page: Page): Promise<void> {
  await page.addInitScript(() => {
    localStorage.setItem('admin-locale', 'en');
  });
}

/**
 * Force portal pt-BR (localStorage + navigator language).
 * Complements playwright.portal.config.ts `locale: 'pt-BR'` for CI en-US browsers.
 */
export async function seedPortalPtBrLocale(page: Page): Promise<void> {
  await page.addInitScript(() => {
    localStorage.setItem('portal-locale', 'pt-BR');
    Object.defineProperty(navigator, 'language', {
      get: () => 'pt-BR',
      configurable: true,
    });
    Object.defineProperty(navigator, 'languages', {
      get: () => ['pt-BR', 'pt'],
      configurable: true,
    });
  });
}
