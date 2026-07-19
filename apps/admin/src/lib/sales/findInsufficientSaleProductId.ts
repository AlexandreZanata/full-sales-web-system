import type { SaleItem } from '@/lib/api/types';
import { fetchStockBalance } from '@/lib/api/inventory';

/** Picks the first sale line that likely lacks stock (tenant available < qty). */
export async function findInsufficientSaleProductId(
  items: SaleItem[],
): Promise<string | undefined> {
  for (const item of items) {
    try {
      const balance = await fetchStockBalance(item.productId);
      if (balance.available < item.quantity) {
        return item.productId;
      }
    } catch {
      // keep scanning other lines
    }
  }
  return items[0]?.productId;
}
