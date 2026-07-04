import type { Page } from '@playwright/test';

/** Force English locale so role-based selectors stay stable in E2E. */
export async function seedEnglishLocale(page: Page): Promise<void> {
  await page.addInitScript(() => {
    localStorage.setItem('admin-locale', 'en');
  });
}
