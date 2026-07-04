import {
  Commerce,
  Cnpj,
  Currency,
  Money,
  Product,
  Sku,
  generateCommerceId,
  generateProductId,
} from '@full-sales/domain';
import type { CommerceId, ProductId, TenantId } from '@full-sales/domain';

import type { CommerceRepository, ProductRepository, SaleRepository } from '../ports/index.js';
import type { Sale } from '@full-sales/domain';
import type { SaleId } from '@full-sales/domain';

export class InMemoryCommerceRepository implements CommerceRepository {
  private readonly items = new Map<string, Commerce>();

  seed(commerce: Commerce): void {
    this.items.set(commerce.id, commerce);
  }

  findById(id: CommerceId, tenantId: TenantId): Promise<Commerce | null> {
    const commerce = this.items.get(id) ?? null;
    if (!commerce || commerce.tenantId !== tenantId) {
      return Promise.resolve(null);
    }
    return Promise.resolve(commerce);
  }
}

export class InMemoryProductRepository implements ProductRepository {
  private readonly items = new Map<string, Product>();

  seed(product: Product): void {
    this.items.set(product.id, product);
  }

  findByIds(ids: ProductId[], tenantId: TenantId): Promise<Product[]> {
    const products = ids
      .map((id) => this.items.get(id))
      .filter((p): p is Product => p !== undefined && p.tenantId === tenantId);
    return Promise.resolve(products);
  }

  list(
    tenantId: TenantId,
    page: number,
    pageSize: number,
  ): Promise<{ items: Product[]; total: number }> {
    const all = [...this.items.values()].filter((p) => p.tenantId === tenantId);
    const start = (page - 1) * pageSize;
    return Promise.resolve({
      items: all.slice(start, start + pageSize),
      total: all.length,
    });
  }
}

export class InMemorySaleRepository implements SaleRepository {
  private readonly items = new Map<string, Sale>();

  save(sale: Sale): Promise<void> {
    this.items.set(sale.id, sale);
    return Promise.resolve();
  }

  findById(id: SaleId, tenantId: TenantId): Promise<Sale | null> {
    const sale = this.items.get(id) ?? null;
    if (!sale || sale.tenantId !== tenantId) {
      return Promise.resolve(null);
    }
    return Promise.resolve(sale);
  }
}

/** Test fixture helpers — contract examples from UC-001. */
export function sampleCommerce(tenantId: TenantId): Commerce {
  return Commerce.create({
    id: generateCommerceId(),
    cnpj: Cnpj.parse('11222333000181'),
    legalName: 'Acme Ltd',
    tenantId,
  });
}

export function sampleProduct(tenantId: TenantId): Product {
  return Product.create({
    id: generateProductId(),
    name: 'Widget',
    sku: Sku.parse('WGT-001'),
    unitPrice: Money.of(1_000, Currency.brl()),
    tenantId,
  });
}
