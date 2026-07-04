/**
 * Contract: Phase 34 sale actions — API error code mapping.
 */
import { describe, expect, it } from 'vitest';

import { saleActionErrorMessage } from '@/lib/sales/saleActionErrors';

describe('saleActionErrorMessage — API-CONTRACT', () => {
  it('when_INSUFFICIENT_STOCK_then_stock_message', () => {
    expect(saleActionErrorMessage('INSUFFICIENT_STOCK')).toContain('Insufficient stock');
  });

  it('when_INVALID_TRANSITION_then_transition_message', () => {
    expect(saleActionErrorMessage('INVALID_TRANSITION')).toContain('not allowed');
  });
});
