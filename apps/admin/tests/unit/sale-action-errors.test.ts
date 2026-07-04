/**
 * Contract: Phase 34 sale actions — API error code → i18n message key mapping.
 */
import { describe, expect, it } from 'vitest';

import { en } from '@/lib/i18n/locales/en';
import { translate } from '@/lib/i18n/translate';
import { saleActionErrorMessage } from '@/lib/sales/saleActionErrors';

const t = (key: Parameters<typeof translate>[1]) => translate(en, key);

describe('saleActionErrorMessage — API-CONTRACT', () => {
  it('when_INSUFFICIENT_STOCK_then_stock_message', () => {
    expect(t(saleActionErrorMessage('INSUFFICIENT_STOCK'))).toContain('Insufficient stock');
  });

  it('when_INVALID_TRANSITION_then_transition_message', () => {
    expect(t(saleActionErrorMessage('INVALID_TRANSITION'))).toContain('not allowed');
  });
});
