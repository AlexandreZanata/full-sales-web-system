/**
 * Contract: Phase 33 — order and delivery status badge tokens cover all states.
 */
import { describe, expect, it } from 'vitest';

import { getDeliveryStatusToken, getOrderStatusToken } from '@/lib/admin-tokens';
import { DELIVERY_STATUSES } from '@/lib/deliveries/constants';
import { ORDER_STATUSES } from '@/lib/orders/constants';

describe('order status tokens — Phase 33 contract', () => {
  it('given_each_order_status_when_get_token_then_has_label', () => {
    for (const status of ORDER_STATUSES) {
      expect(getOrderStatusToken(status).label.length).toBeGreaterThan(0);
    }
  });
});

describe('delivery status tokens — Phase 33 contract', () => {
  it('given_each_delivery_status_when_get_token_then_has_label', () => {
    for (const status of DELIVERY_STATUSES) {
      expect(getDeliveryStatusToken(status).label.length).toBeGreaterThan(0);
    }
  });
});
