import type { ProductId, TenantId } from '../value-objects/ids.js';
import { Money } from '../value-objects/money.js';
import type { Sku } from '../value-objects/sku.js';

export interface ProductCreateInput {
  readonly id: ProductId;
  readonly name: string;
  readonly sku: Sku;
  readonly unitPrice: Money;
  readonly tenantId: TenantId;
  readonly active?: boolean;
}

/** Sellable SKU with name, identifier, and unit price. */
export class Product {
  private constructor(
    readonly id: ProductId,
    readonly name: string,
    readonly sku: Sku,
    readonly unitPrice: Money,
    readonly tenantId: TenantId,
    private readonly _active: boolean,
  ) {}

  static create(input: ProductCreateInput): Product {
    return new Product(
      input.id,
      input.name.trim(),
      input.sku,
      input.unitPrice,
      input.tenantId,
      input.active ?? true,
    );
  }

  isActive(): boolean {
    return this._active;
  }

  deactivate(): Product {
    return new Product(this.id, this.name, this.sku, this.unitPrice, this.tenantId, false);
  }
}
