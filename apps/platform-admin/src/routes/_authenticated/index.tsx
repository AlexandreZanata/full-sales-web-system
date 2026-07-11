import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import {
  AlertTriangle,
  Building2,
  Clock,
  DollarSign,
  PauseCircle,
  Plus,
  Activity,
} from 'lucide-react';
import { useMemo } from 'react';

import { FraudSeverityChart } from '@/components/charts/FraudSeverityChart';
import { HealthUptimeChart } from '@/components/charts/HealthUptimeChart';
import {
  TenantStatusDonutChart,
  type TenantStatusSlice,
} from '@/components/charts/TenantStatusDonutChart';
import { KpiStatCard } from '@/components/dashboard/KpiStatCard';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchDashboardSummary } from '@/lib/api/dashboard';
import { fetchFraudEvents } from '@/lib/api/fraud';
import { fetchHealthMatrix } from '@/lib/api/health';
import { chartColors, fraudSeverityColor } from '@/lib/chart-colors';
import { formatDateTime } from '@/lib/formatDateTime';
import { formatMoneyMinor } from '@/lib/i18n/labels';
import { useI18n } from '@/lib/i18n/context';
import { tenantStatusTone } from '@/lib/platform-tokens';

export const Route = createFileRoute('/_authenticated/')({
  component: DashboardPage,
});

function DashboardPage() {
  const { t } = useI18n();
  const summary = useQuery({ queryKey: ['dashboard', 'summary'], queryFn: fetchDashboardSummary });
  const health = useQuery({ queryKey: ['health', 'matrix'], queryFn: fetchHealthMatrix });
  const fraud = useQuery({
    queryKey: ['fraud', 'recent'],
    queryFn: () => fetchFraudEvents({ limit: 5, status: 'open' }),
  });

  const tenantSlices = useMemo((): TenantStatusSlice[] => {
    const data = summary.data;
    if (!data) return [];
    return [
      {
        key: 'active',
        label: t('dashboard.activeTenants'),
        value: data.active,
        color: chartColors.active,
      },
      {
        key: 'trial',
        label: t('dashboard.trialTenants'),
        value: data.trial,
        color: chartColors.trial,
      },
      {
        key: 'pastDue',
        label: t('dashboard.pastDue'),
        value: data.pastDue,
        color: chartColors.pastDue,
      },
      {
        key: 'suspended',
        label: t('dashboard.suspended'),
        value: data.suspended,
        color: chartColors.suspended,
      },
    ];
  }, [summary.data, t]);

  const healthRows = useMemo(() => {
    if (!health.data) return [];
    return Object.entries(health.data.probes).map(([name, probe]) => ({
      name,
      uptime: probe.uptime24hPct,
      status: probe.status,
    }));
  }, [health.data]);

  const fraudSeverityRows = useMemo(() => {
    const events = fraud.data?.data ?? [];
    const counts = new Map<string, number>();
    for (const event of events) {
      counts.set(event.severity, (counts.get(event.severity) ?? 0) + 1);
    }
    return [...counts.entries()].map(([severity, count]) => ({ severity, count }));
  }, [fraud.data]);

  if (summary.isLoading) {
    return <LoadingSpinner />;
  }

  const data = summary.data;

  return (
    <div className="space-y-6">
      <PageHeader title={t('dashboard.title')} description={t('dashboard.subtitle')} />

      <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-5">
        <KpiStatCard
          label={t('dashboard.activeTenants')}
          value={data?.active ?? 0}
          icon={Building2}
          accentClass="bg-status-active"
          valueClass={tenantStatusTone('Active')}
        />
        <KpiStatCard
          label={t('dashboard.trialTenants')}
          value={data?.trial ?? 0}
          icon={Clock}
          accentClass="bg-status-info"
          valueClass={tenantStatusTone('Trial')}
        />
        <KpiStatCard
          label={t('dashboard.pastDue')}
          value={data?.pastDue ?? 0}
          icon={AlertTriangle}
          accentClass="bg-status-warning"
          valueClass={tenantStatusTone('PastDue')}
        />
        <KpiStatCard
          label={t('dashboard.suspended')}
          value={data?.suspended ?? 0}
          icon={PauseCircle}
          accentClass="bg-status-out-of-stock"
          valueClass={tenantStatusTone('Suspended')}
        />
        <KpiStatCard
          label={t('dashboard.mrr')}
          value={data ? formatMoneyMinor(data.mrrMinor, data.mrrCurrency) : '—'}
          icon={DollarSign}
          accentClass="bg-primary"
        />
      </div>

      <div className="flex flex-wrap gap-2">
        <Link to="/tenants/new">
          <Button className="gap-2">
            <Plus className="size-4" aria-hidden />
            {t('dashboard.createTenant')}
          </Button>
        </Link>
        <Link to="/health">
          <Button variant="secondary" className="gap-2">
            <Activity className="size-4" aria-hidden />
            {t('dashboard.viewHealth')}
          </Button>
        </Link>
      </div>

      <div className="grid gap-6 lg:grid-cols-3">
        <Card className="p-5 lg:col-span-1">
          <h2 className="text-sm font-semibold text-foreground">
            {t('dashboard.tenantDistribution')}
          </h2>
          <p className="mt-1 text-xs text-muted-foreground">
            {t('dashboard.tenantDistributionHint')}
          </p>
          <TenantStatusDonutChart
            slices={tenantSlices}
            centerLabel={t('dashboard.totalTenants')}
            centerValue={data?.total ?? 0}
          />
          <ul className="mt-2 grid grid-cols-2 gap-2 text-xs">
            {tenantSlices.map((slice) => (
              <li key={slice.key} className="flex items-center gap-2">
                <span className="size-2 rounded-full" style={{ backgroundColor: slice.color }} />
                <span className="text-muted-foreground">{slice.label}</span>
                <span className="ml-auto font-medium tabular-nums">{slice.value}</span>
              </li>
            ))}
          </ul>
        </Card>

        <Card className="p-5 lg:col-span-2">
          <h2 className="text-sm font-semibold text-foreground">{t('dashboard.healthUptime')}</h2>
          <p className="mt-1 text-xs text-muted-foreground">{t('dashboard.healthUptimeHint')}</p>
          {health.isLoading ? <LoadingSpinner /> : null}
          {healthRows.length ? (
            <HealthUptimeChart rows={healthRows} uptimeLabel={t('dashboard.uptime24h')} />
          ) : (
            <EmptyState title={t('common.noResults')} />
          )}
        </Card>
      </div>

      <div className="grid gap-6 lg:grid-cols-2">
        <Card className="p-5">
          <div className="mb-4 flex items-center justify-between">
            <div>
              <h2 className="text-sm font-semibold text-foreground">
                {t('dashboard.recentFraud')}
              </h2>
              <p className="mt-1 text-xs text-muted-foreground">{t('dashboard.fraudHint')}</p>
            </div>
            <Link
              to="/fraud"
              className="text-sm font-medium text-primary underline-offset-2 hover:underline"
            >
              {t('common.viewAll')}
            </Link>
          </div>
          {fraud.isLoading ? <LoadingSpinner /> : null}
          {fraudSeverityRows.length ? (
            <FraudSeverityChart rows={fraudSeverityRows} eventsLabel={t('dashboard.events')} />
          ) : null}
          {fraud.data?.data.length ? (
            <ul className="mt-4 divide-y divide-hairline">
              {fraud.data.data.map((event) => (
                <li key={event.id} className="flex gap-3 py-3 text-sm first:pt-0">
                  <span
                    className="mt-1.5 size-2 shrink-0 rounded-full"
                    style={{ backgroundColor: fraudSeverityColor(event.severity) }}
                  />
                  <div className="min-w-0 flex-1">
                    <p className="font-medium text-foreground">{event.eventType}</p>
                    <p className="text-muted-foreground">
                      {event.severity} · {formatDateTime(event.createdAt)}
                    </p>
                  </div>
                </li>
              ))}
            </ul>
          ) : (
            <EmptyState title={t('common.noResults')} />
          )}
        </Card>

        <Card className="p-5">
          <h2 className="text-sm font-semibold text-foreground">{t('dashboard.healthMatrix')}</h2>
          <p className="mt-1 text-xs text-muted-foreground">{t('dashboard.healthMatrixHint')}</p>
          {health.isLoading ? <LoadingSpinner /> : null}
          {health.data ? (
            <ul className="mt-4 space-y-3">
              {Object.entries(health.data.probes).map(([name, probe]) => (
                <li key={name}>
                  <div className="mb-1 flex items-center justify-between text-sm">
                    <span className="font-medium">{name}</span>
                    <span className="text-muted-foreground">{probe.uptime24hPct.toFixed(1)}%</span>
                  </div>
                  <div className="h-2 overflow-hidden rounded-full bg-surface-muted">
                    <div
                      className="h-full rounded-full transition-all"
                      style={{
                        width: `${String(Math.min(probe.uptime24hPct, 100))}%`,
                        backgroundColor:
                          probe.uptime24hPct >= 99
                            ? chartColors.active
                            : probe.uptime24hPct >= 95
                              ? chartColors.pastDue
                              : chartColors.fraud.high,
                      }}
                    />
                  </div>
                </li>
              ))}
            </ul>
          ) : null}
        </Card>
      </div>
    </div>
  );
}
