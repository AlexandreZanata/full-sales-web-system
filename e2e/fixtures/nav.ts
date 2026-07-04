import type { Page } from '@playwright/test';

const MOBILE_MAX_WIDTH = 767;

function isMobileViewport(page: Page): boolean {
  const width = page.viewportSize()?.width ?? 1280;
  return width <= MOBILE_MAX_WIDTH;
}

export async function openNavLink(page: Page, label: string): Promise<void> {
  if (isMobileViewport(page)) {
    await page.getByRole('button', { name: 'Open navigation menu' }).click();
    await page
      .getByRole('dialog', { name: 'Navigation menu' })
      .getByRole('link', { name: label })
      .click();
    return;
  }

  await page
    .getByRole('navigation', { name: 'Admin navigation' })
    .getByRole('link', { name: label })
    .click();
}
