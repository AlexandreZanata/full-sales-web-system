/**
 * Contract: Phase 34 — payment and declared payment badge helpers (i18n keys).
 */
import { describe, expect, it } from 'vitest';

import { en } from '@/lib/i18n/locales/en';
import { translateDeclaredPayment, translatePaymentMethod } from '@/lib/i18n/labels';
import { translate } from '@/lib/i18n/translate';
import { isDeclaredPayment } from '@/lib/sales/constants';

const t = (key: Parameters<typeof translate>[1]) => translate(en, key);

describe('translatePaymentMethod — ADR-006 contract', () => {
  it('when_cash_then_cash_label', () => {
    expect(translatePaymentMethod(t, 'Cash')).toBe('Cash');
    expect(translatePaymentMethod(t, 'cash')).toBe('Cash');
  });
});

describe('isDeclaredPayment — RN-PAG1 contract', () => {
  it('when_not_declared_then_false', () => {
    expect(isDeclaredPayment('NotDeclared', false)).toBe(false);
    expect(isDeclaredPayment('', false)).toBe(false);
  });

  it('when_received_then_true', () => {
    expect(isDeclaredPayment('NotDeclared', true)).toBe(true);
  });

  it('when_pix_declared_then_true', () => {
    expect(isDeclaredPayment('Pix', false)).toBe(true);
    expect(translateDeclaredPayment(t, 'Pix', true)).toContain('received');
  });
});
