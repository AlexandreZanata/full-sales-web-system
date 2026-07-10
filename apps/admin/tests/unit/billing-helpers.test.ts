/**
 * Contract: docs/API-CONTRACT.md — GET /v1/billing/subscription includes tenantStatus (Phase 12).
 */
import { describe, expect, it } from 'vitest';

import {
  UPGRADE_PLAN_MAILTO,
  daysUntil,
  isStarterPlan,
  shouldShowBillingBanner,
} from '@/lib/billing/helpers';
import type { Subscription } from '@/lib/api/billing';

function sampleSubscription(overrides: Partial<Subscription> = {}): Subscription {
  return {
    plan: {
      id: '550e8400-e29b-41d4-a716-446655440000',
      code: 'pro',
      name: 'Pro',
      priceMinor: 9900,
      billingInterval: 'month',
    },
    status: 'Active',
    tenantStatus: 'Active',
    ...overrides,
  };
}

describe('billing helpers — Phase 12 contract', () => {
  it('given_starter_plan_when_isStarterPlan_then_true', () => {
    expect(isStarterPlan('starter')).toBe(true);
    expect(isStarterPlan('Pro')).toBe(false);
  });

  it('given_past_due_when_shouldShowBillingBanner_then_true', () => {
    expect(shouldShowBillingBanner(sampleSubscription({ tenantStatus: 'PastDue' }))).toBe(true);
    expect(shouldShowBillingBanner(sampleSubscription({ status: 'PastDue' }))).toBe(true);
  });

  it('given_suspended_tenant_when_shouldShowBillingBanner_then_true', () => {
    expect(shouldShowBillingBanner(sampleSubscription({ tenantStatus: 'Suspended' }))).toBe(true);
  });

  it('given_active_when_shouldShowBillingBanner_then_false', () => {
    expect(shouldShowBillingBanner(sampleSubscription())).toBe(false);
  });

  it('upgrade_mailto_is_mailto_link', () => {
    expect(UPGRADE_PLAN_MAILTO.startsWith('mailto:')).toBe(true);
  });

  it('daysUntil_never_negative', () => {
    const future = new Date(Date.now() + 3 * 24 * 60 * 60 * 1000).toISOString();
    expect(daysUntil(future)).toBeGreaterThanOrEqual(3);
  });
});
