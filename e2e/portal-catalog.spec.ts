import { expect, test } from '@playwright/test';

import { mockPortalApi } from './fixtures/portal-api-mock';
import { MOCK_CATEGORY, MOCK_PRODUCT } from './fixtures/portal-catalog-mock';

test.describe('Portal catalog flow', () => {
  test('given_categories_when_select_and_search_then_add_to_cart', async ({ page }) => {
    await mockPortalApi(page);
    await page.goto('/');

    await expect(page).toHaveURL(new RegExp(`category=${MOCK_CATEGORY.slug}`));
    await expect(page.getByRole('tab', { name: MOCK_CATEGORY.name })).toHaveAttribute(
      'aria-current',
      'true',
    );
    await expect(page.getByText(MOCK_PRODUCT.name)).toBeVisible();

    await page.getByLabel('Buscar').fill('missing');
    await expect(page.getByText('Nenhum produto encontrado.')).toBeVisible();

    await page.getByLabel('Buscar').fill(MOCK_PRODUCT.sku);
    await expect(page.getByText(MOCK_PRODUCT.name)).toBeVisible();

    await page.getByRole('button', { name: 'Visualização em lista' }).click();
    await expect(page.getByTestId('catalog-product-list')).toBeVisible();

    await page.getByRole('button', { name: 'Adicionar ao carrinho' }).first().click();
    await page.getByRole('link', { name: /Carrinho/ }).click();
    await expect(page).toHaveURL('/cart');
    await expect(page.getByText(MOCK_PRODUCT.name)).toBeVisible();
  });
});
