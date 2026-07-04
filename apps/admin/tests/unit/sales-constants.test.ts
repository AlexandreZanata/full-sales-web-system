/**
 * Contract: Phase 34 — payment and declared payment badge helpers.
 */
import { describe, expect, it } from 'vitest';

import { declaredPaymentLabel, isDeclaredPayment, paymentMethodLabel } from '@/lib/sales/constants';

describe('paymentMethodLabel — ADR-006 contract', () => {
  it('when_cash_then_cash_label', () => {
    expect(paymentMethodLabel('Cash')).toBe('Cash');
    expect(paymentMethodLabel('cash')).toBe('Cash');
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
    expect(declaredPaymentLabel('Pix', true)).toContain('received');
  });
});
