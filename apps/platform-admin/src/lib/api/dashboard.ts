import { fetchAllCursorPages } from '@/lib/cursorPagination';
import { fetchTenants } from '@/lib/api/tenants';
import { fetchTenantStats } from '@/lib/api/users';

export type DashboardSummary = {
  active: number;
  trial: number;
  pastDue: number;
  suspended: number;
  total: number;
  mrrMinor: number;
  mrrCurrency: string;
};

export async function fetchDashboardSummary(): Promise<DashboardSummary> {
  const tenants = await fetchAllCursorPages((cursor) => fetchTenants({ limit: 100, cursor }));
  const summary: DashboardSummary = {
    active: 0,
    trial: 0,
    pastDue: 0,
    suspended: 0,
    total: tenants.length,
    mrrMinor: 0,
    mrrCurrency: 'BRL',
  };

  for (const tenant of tenants) {
    switch (tenant.status) {
      case 'Active':
        summary.active += 1;
        break;
      case 'Trial':
        summary.trial += 1;
        break;
      case 'PastDue':
        summary.pastDue += 1;
        break;
      case 'Suspended':
        summary.suspended += 1;
        break;
      default:
        break;
    }
  }

  const billable = tenants
    .filter((t) => t.status === 'Active' || t.status === 'PastDue')
    .slice(0, 25);
  for (const tenant of billable) {
    try {
      const stats = await fetchTenantStats(tenant.id);
      summary.mrrMinor += stats.mrrMinor;
      summary.mrrCurrency = stats.mrrCurrency;
    } catch {
      // ponytail: skip tenants without stats
    }
  }

  return summary;
}
