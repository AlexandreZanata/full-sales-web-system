import type { Commerce } from '@full-sales/domain';
import type { Product } from '@full-sales/domain';
import type { Sale } from '@full-sales/domain';
import type { CommerceId, ProductId, SaleId, TenantId } from '@full-sales/domain';

/** Port: load commerce by id for sale creation. */
export interface CommerceRepository {
  findById(id: CommerceId, tenantId: TenantId): Promise<Commerce | null>;
}

/** Port: load products and list catalog. */
export interface ProductRepository {
  findByIds(ids: ProductId[], tenantId: TenantId): Promise<Product[]>;
  list(
    tenantId: TenantId,
    page: number,
    pageSize: number,
  ): Promise<{
    items: Product[];
    total: number;
  }>;
}

/** Port: persist and load sales. */
export interface SaleRepository {
  save(sale: Sale): Promise<void>;
  findById(id: SaleId, tenantId: TenantId): Promise<Sale | null>;
}
