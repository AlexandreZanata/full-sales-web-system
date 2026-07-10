import { expect, test } from '@playwright/test';

import { mockPortalApi } from './fixtures/portal-api-mock';
import { MOCK_CATEGORY, MOCK_PRODUCT } from './fixtures/portal-catalog-mock';

test.describe('Portal catalog flow', () => {
  test('given_home_when_load_then_all_sections_visible', async ({ page }) => {
    await mockPortalApi(page);
    await page.goto('/');

    await expect(page.getByTestId('catalog-home-page')).toBeVisible();
    await expect(page.getByTestId('hero-banner')).toBeVisible();
    await expect(page.getByTestId('home-categories')).toBeVisible();
    await expect(page.getByTestId('featured-items')).toBeVisible();
    await expect(page.getByText('Featured Burger')).toBeVisible();
    await expect(page.getByTestId('popular-items')).toBeVisible();
    await expect(page.getByText('Popular Soda')).toBeVisible();
  });

  test('given_categories_when_select_and_search_then_add_to_cart', async ({ page }) => {
    await mockPortalApi(page);
    await page.goto('/');

    await page.getByLabel('Main').getByRole('link', { name: 'Cardápio', exact: true }).click();
    await expect(page).toHaveURL(new RegExp(`category=${MOCK_CATEGORY.slug}`));
    await expect(page.getByRole('tab', { name: MOCK_CATEGORY.name })).toHaveAttribute(
      'aria-current',
      'true',
    );
    await expect(page.getByText(MOCK_PRODUCT.name)).toBeVisible();

    const menuSearch = page.getByTestId('catalog-menu').getByRole('searchbox', { name: 'Buscar' });
    await menuSearch.fill('missing');
    await expect(page.getByText('Nenhum produto encontrado.')).toBeVisible();

    await menuSearch.fill(MOCK_PRODUCT.sku);
    await expect(page.getByText(MOCK_PRODUCT.name)).toBeVisible();

    await page.getByRole('button', { name: 'Visualização em lista' }).click();
    await expect(page.getByTestId('catalog-product-list')).toBeVisible();

    await page.getByTestId('catalog-menu').getByRole('button', { name: 'Adicionar ao carrinho' }).first().click();
    await page.locator('a.portal-cart-pill').filter({ visible: true }).click();
    await expect(page).toHaveURL('/cart');
    await expect(page.getByText(MOCK_PRODUCT.name)).toBeVisible();
  });

  test('given_product_with_gallery_when_open_detail_then_carousel_visible', async ({ page }) => {
    await mockPortalApi(page);
    await page.goto('/');

    await page.getByLabel('Main').getByRole('link', { name: 'Cardápio', exact: true }).click();
    await expect(page).toHaveURL(new RegExp(`category=${MOCK_CATEGORY.slug}`));

    await page.getByTestId('catalog-menu').getByLabel(MOCK_PRODUCT.name).click();
    await expect(page).toHaveURL(
      new RegExp(`/products/${MOCK_PRODUCT.id}\\?category=${MOCK_CATEGORY.slug}`),
    );
    await expect(page.getByRole('img', { name: MOCK_PRODUCT.name })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Próxima imagem' })).toBeVisible();
    await expect(page.getByText('Unidade de medida')).toBeVisible();
    await expect(page.getByText('UN', { exact: true })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Falar com vendedor' })).toHaveAttribute(
      'href',
      /wa\.me\/5511987654321/,
    );
  });
});
