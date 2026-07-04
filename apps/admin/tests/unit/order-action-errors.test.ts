/**
 * Contract: Phase 33 order actions — API error code → i18n message key mapping.
 */
import { describe, expect, it } from 'vitest';

import { en } from '@/lib/i18n/locales/en';
import { orderActionErrorKey } from '@/lib/i18n/labels';
import { translate } from '@/lib/i18n/translate';
import { orderActionErrorMessage } from '@/lib/orders/orderActionErrors';

const t = (key: Parameters<typeof translate>[1]) => translate(en, key);

describe('orderActionErrorMessage — API-CONTRACT', () => {
  it('when_INSUFFICIENT_STOCK_then_stock_message', () => {
    expect(t(orderActionErrorMessage('INSUFFICIENT_STOCK'))).toContain('Insufficient stock');
  });

  it('when_INVALID_ORDER_TRANSITION_then_transition_message', () => {
    expect(t(orderActionErrorMessage('INVALID_ORDER_TRANSITION'))).toContain('not allowed');
  });

  it('when_REJECTION_REASON_REQUIRED_then_reason_message', () => {
    expect(t(orderActionErrorMessage('REJECTION_REASON_REQUIRED'))).toContain('reason');
  });

  it('when_unknown_code_then_action_failed_key', () => {
    expect(orderActionErrorMessage('UNKNOWN')).toBe('errors.actionFailed');
    expect(orderActionErrorKey('UNKNOWN')).toBe('errors.actionFailed');
  });
});
