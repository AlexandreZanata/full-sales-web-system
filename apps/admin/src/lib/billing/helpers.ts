import type { Subscription } from '@/lib/api/billing';

const STARTER_CODES = new Set(['starter', 'free']);

export function isStarterPlan(planCode: string): boolean {
  return STARTER_CODES.has(planCode.trim().toLowerCase());
}

export function shouldShowBillingBanner(subscription: Subscription): boolean {
  const tenant = subscription.tenantStatus;
  const sub = subscription.status;
  return tenant === 'PastDue' || tenant === 'Suspended' || sub === 'PastDue';
}

export const UPGRADE_PLAN_MAILTO =
  'mailto:support@fullsales.local?subject=Plan%20upgrade%20request';

export function formatMoneyMinor(amountMinor: number, currency: string): string {
  return new Intl.NumberFormat(undefined, {
    style: 'currency',
    currency,
  }).format(amountMinor / 100);
}

export function daysUntil(isoDate: string): number {
  const target = new Date(isoDate).getTime();
  const now = Date.now();
  return Math.max(0, Math.ceil((target - now) / (24 * 60 * 60 * 1000)));
}
