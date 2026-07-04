import { describe, expect, it } from 'vitest';

import { addressesForCommerce, orderStatusIndex } from '@/lib/orders/constants';

describe('addressesForCommerce', () => {
  it('returns delivery addresses for seed commerce', () => {
    const addresses = addressesForCommerce('01900001-0010-7000-8000-000000000001');
    expect(addresses.length).toBeGreaterThan(0);
    expect(addresses.every((address) => address.type === 'Delivery')).toBe(true);
  });
});

describe('orderStatusIndex', () => {
  it('maps PendingApproval to timeline index', () => {
    expect(orderStatusIndex('PendingApproval')).toBe(1);
  });

  it('returns -1 for unknown status', () => {
    expect(orderStatusIndex('Unknown')).toBe(-1);
  });
});
