import { describe, expect, it } from 'vitest';

import { generateSaleId, generateTenantId, generateUserId } from '@full-sales/domain';

import {
  InMemoryCommerceRepository,
  InMemoryProductRepository,
  InMemorySaleRepository,
  sampleCommerce,
  sampleProduct,
} from '../adapters/in-memory.js';
import { CreateSaleHandler, GetSaleHandler, ListProductsHandler } from '../handlers/sales.js';

// Contract: UC-001 step 4 — create sale in Pending with server-computed total (BR-SA-002)
describe('CreateSaleHandler', () => {
  it('given_valid_items_when_create_then_pending_sale_with_total_from_items', async () => {
    const tenantId = generateTenantId();
    const commerceRepo = new InMemoryCommerceRepository();
    const productRepo = new InMemoryProductRepository();
    const saleRepo = new InMemorySaleRepository();
    const commerce = sampleCommerce(tenantId);
    const product = sampleProduct(tenantId);
    commerceRepo.seed(commerce);
    productRepo.seed(product);

    const handler = new CreateSaleHandler(commerceRepo, productRepo, saleRepo);
    const saleId = generateSaleId();
    const result = await handler.execute(
      {
        commerceId: commerce.id,
        items: [{ productId: product.id, quantity: 2 }],
        paymentMethod: 'Cash',
      },
      generateUserId(),
      tenantId,
      saleId,
    );

    expect(result.status).toBe('Pending');
    expect(result.totalAmount).toBe(2_000);
    expect(result.totalCurrency).toBe('BRL');
  });

  it('given_invalid_quantity_when_create_then_throws', async () => {
    const tenantId = generateTenantId();
    const commerceRepo = new InMemoryCommerceRepository();
    const productRepo = new InMemoryProductRepository();
    const saleRepo = new InMemorySaleRepository();
    const commerce = sampleCommerce(tenantId);
    const product = sampleProduct(tenantId);
    commerceRepo.seed(commerce);
    productRepo.seed(product);

    const handler = new CreateSaleHandler(commerceRepo, productRepo, saleRepo);
    await expect(
      handler.execute(
        {
          commerceId: commerce.id,
          items: [{ productId: product.id, quantity: 0 }],
          paymentMethod: 'Pix',
        },
        generateUserId(),
        tenantId,
        generateSaleId(),
      ),
    ).rejects.toThrow();
  });
});

describe('GetSaleHandler', () => {
  it('given_unknown_id_when_get_then_null', async () => {
    const handler = new GetSaleHandler(new InMemorySaleRepository());
    const result = await handler.execute(generateSaleId(), generateTenantId());
    expect(result).toBeNull();
  });
});

describe('ListProductsHandler', () => {
  it('given_products_when_list_then_pagination_meta', async () => {
    const tenantId = generateTenantId();
    const productRepo = new InMemoryProductRepository();
    productRepo.seed(sampleProduct(tenantId));

    const handler = new ListProductsHandler(productRepo);
    const result = await handler.execute(tenantId, 1, 20);

    expect(result.page).toBe(1);
    expect(result.pageSize).toBe(20);
    expect(result.total).toBe(1);
    expect(result.items[0]?.sku).toBe('WGT-001');
  });
});
