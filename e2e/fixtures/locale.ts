import type { Page } from '@playwright/test';

/** Force English locale so admin role-based selectors stay stable in E2E. */
export async function seedEnglishLocale(page: Page): Promise<void> {
  await page.addInitScript(() => {
    localStorage.setItem('admin-locale', 'en');
  });
}

/**
 * Force portal pt-BR so selectors like "Cardápio" / "Carrinho" match CI runners
 * whose browser language is en-US (default would be English "Menu").
 */
export async function seedPortalPtBrLocale(page: Page): Promise<void> {
  await page.addInitScript(() => {
    localStorage.setItem('portal-locale', 'pt-BR');
  });
}
