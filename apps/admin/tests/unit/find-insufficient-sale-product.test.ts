/**
 * Contract: when confirm fails with INSUFFICIENT_STOCK, pick a short product for stock CTA.
 */
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { findInsufficientSaleProductId } from '@/lib/sales/findInsufficientSaleProductId';

const fetchStockBalance = vi.fn();

vi.mock('@/lib/api/inventory', () => ({
  fetchStockBalance: (...args: unknown[]) => fetchStockBalance(...args),
}));

describe('findInsufficientSaleProductId', () => {
  beforeEach(() => {
    fetchStockBalance.mockReset();
  });

  it('given_line_below_available_when_scan_then_returns_that_product', async () => {
    fetchStockBalance.mockResolvedValueOnce({ productId: 'p1', available: 0 });
    const id = await findInsufficientSaleProductId([
      {
        productId: 'p1',
        quantity: 2,
        unitPriceAmount: 1,
        unitPriceCurrency: 'BRL',
        lineTotalAmount: 2,
      },
    ]);
    expect(id).toBe('p1');
  });

  it('given_first_ok_second_short_when_scan_then_returns_second', async () => {
    fetchStockBalance
      .mockResolvedValueOnce({ productId: 'p1', available: 5 })
      .mockResolvedValueOnce({ productId: 'p2', available: 0 });
    const id = await findInsufficientSaleProductId([
      {
        productId: 'p1',
        quantity: 1,
        unitPriceAmount: 1,
        unitPriceCurrency: 'BRL',
        lineTotalAmount: 1,
      },
      {
        productId: 'p2',
        quantity: 3,
        unitPriceAmount: 1,
        unitPriceCurrency: 'BRL',
        lineTotalAmount: 3,
      },
    ]);
    expect(id).toBe('p2');
  });

  it('given_balance_lookup_fails_when_scan_then_falls_back_to_first_item', async () => {
    fetchStockBalance.mockRejectedValue(new Error('network'));
    const id = await findInsufficientSaleProductId([
      {
        productId: 'fallback',
        quantity: 1,
        unitPriceAmount: 1,
        unitPriceCurrency: 'BRL',
        lineTotalAmount: 1,
      },
    ]);
    expect(id).toBe('fallback');
  });
});
