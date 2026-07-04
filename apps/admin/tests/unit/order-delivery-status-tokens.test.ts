/**
 * Contract: Phase 33 — order and delivery status badge tokens cover all states.
 */
import { describe, expect, it } from 'vitest';

import { getDeliveryStatusToken, getOrderStatusToken } from '@/lib/admin-tokens';
import { DELIVERY_STATUSES } from '@/lib/deliveries/constants';
import { en } from '@/lib/i18n/locales/en';
import { translateDeliveryStatus, translateOrderStatus } from '@/lib/i18n/labels';
import { translate } from '@/lib/i18n/translate';
import { ORDER_STATUSES } from '@/lib/orders/constants';

const t = (key: Parameters<typeof translate>[1]) => translate(en, key);

describe('order status tokens — Phase 33 contract', () => {
  it('given_each_order_status_when_get_token_then_has_visual_styles', () => {
    for (const status of ORDER_STATUSES) {
      const token = getOrderStatusToken(status);
      expect(token.dot.length).toBeGreaterThan(0);
      expect(token.badge.length).toBeGreaterThan(0);
    }
  });

  it('given_each_order_status_when_translate_then_has_label', () => {
    for (const status of ORDER_STATUSES) {
      expect(translateOrderStatus(t, status).length).toBeGreaterThan(0);
    }
  });
});

describe('delivery status tokens — Phase 33 contract', () => {
  it('given_each_delivery_status_when_get_token_then_has_visual_styles', () => {
    for (const status of DELIVERY_STATUSES) {
      const token = getDeliveryStatusToken(status);
      expect(token.dot.length).toBeGreaterThan(0);
      expect(token.badge.length).toBeGreaterThan(0);
    }
  });

  it('given_each_delivery_status_when_translate_then_has_label', () => {
    for (const status of DELIVERY_STATUSES) {
      expect(translateDeliveryStatus(t, status).length).toBeGreaterThan(0);
    }
  });
});
