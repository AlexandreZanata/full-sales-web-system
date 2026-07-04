import {
  Quantity,
  Sale,
  parsePaymentMethod,
  type CommerceId,
  type ProductId,
  type SaleId,
  type TenantId,
  type UserId,
} from '@full-sales/domain';

import type { CommerceRepository, ProductRepository, SaleRepository } from '../ports/index.js';

export interface CreateSaleInput {
  readonly commerceId: CommerceId;
  readonly items: ReadonlyArray<{ productId: ProductId; quantity: number }>;
  readonly paymentMethod: string;
}

export interface CreateSaleOutput {
  readonly id: SaleId;
  readonly status: string;
  readonly totalAmount: number;
  readonly totalCurrency: string;
}

export class CreateSaleHandler {
  constructor(
    private readonly commerces: CommerceRepository,
    private readonly products: ProductRepository,
    private readonly sales: SaleRepository,
  ) {}

  async execute(
    input: CreateSaleInput,
    driverId: UserId,
    tenantId: TenantId,
    saleId: SaleId,
  ): Promise<CreateSaleOutput> {
    const commerce = await this.commerces.findById(input.commerceId, tenantId);
    if (!commerce) {
      throw new Error('COMMERCE_NOT_FOUND');
    }

    const productIds = input.items.map((item) => item.productId);
    const products = await this.products.findByIds(productIds, tenantId);
    const paymentMethod = parsePaymentMethod(input.paymentMethod);

    let sale = Sale.create({
      id: saleId,
      driverId,
      commerce,
      paymentMethod,
      tenantId,
    });

    for (const line of input.items) {
      const product = products.find((p) => p.id === line.productId);
      if (!product) {
        throw new Error('PRODUCT_NOT_FOUND');
      }
      sale = sale.addItem({ product, quantity: Quantity.of(line.quantity) });
    }

    await this.sales.save(sale);
    const total = sale.total();
    return {
      id: sale.id,
      status: sale.status,
      totalAmount: total.amountMinor,
      totalCurrency: total.currency.toString(),
    };
  }
}

export interface GetSaleOutput {
  readonly id: SaleId;
  readonly status: string;
  readonly totalAmount: number;
  readonly totalCurrency: string;
}

export class GetSaleHandler {
  constructor(private readonly sales: SaleRepository) {}

  async execute(id: SaleId, tenantId: TenantId): Promise<GetSaleOutput | null> {
    const sale = await this.sales.findById(id, tenantId);
    if (!sale) {
      return null;
    }
    const total = sale.total();
    return {
      id: sale.id,
      status: sale.status,
      totalAmount: total.amountMinor,
      totalCurrency: total.currency.toString(),
    };
  }
}

export interface ListProductsOutput {
  readonly items: ReadonlyArray<{
    id: ProductId;
    name: string;
    sku: string;
    priceAmount: number;
    priceCurrency: string;
    active: boolean;
  }>;
  readonly page: number;
  readonly pageSize: number;
  readonly total: number;
}

export class ListProductsHandler {
  constructor(private readonly products: ProductRepository) {}

  async execute(tenantId: TenantId, page: number, pageSize: number): Promise<ListProductsOutput> {
    const clampedSize = Math.min(Math.max(pageSize, 1), 50);
    const result = await this.products.list(tenantId, Math.max(page, 1), clampedSize);
    return {
      items: result.items.map((product) => ({
        id: product.id,
        name: product.name,
        sku: product.sku.toString(),
        priceAmount: product.unitPrice.amountMinor,
        priceCurrency: product.unitPrice.currency.toString(),
        active: product.isActive(),
      })),
      page: Math.max(page, 1),
      pageSize: clampedSize,
      total: result.total,
    };
  }
}
