/**
 * Contract: Phase 33 order actions — API error code → user message mapping.
 */
import { describe, expect, it } from 'vitest';

import { orderActionErrorMessage } from '@/lib/orders/orderActionErrors';

describe('orderActionErrorMessage — API-CONTRACT', () => {
  it('when_INSUFFICIENT_STOCK_then_stock_message', () => {
    expect(orderActionErrorMessage('INSUFFICIENT_STOCK')).toContain('Insufficient stock');
  });

  it('when_INVALID_ORDER_TRANSITION_then_transition_message', () => {
    expect(orderActionErrorMessage('INVALID_ORDER_TRANSITION')).toContain('not allowed');
  });

  it('when_REJECTION_REASON_REQUIRED_then_reason_message', () => {
    expect(orderActionErrorMessage('REJECTION_REASON_REQUIRED')).toContain('reason');
  });
});
