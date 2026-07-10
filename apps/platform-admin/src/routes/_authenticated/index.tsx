import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchDashboardSummary } from '@/lib/api/dashboard';
import { fetchFraudEvents } from '@/lib/api/fraud';
import { fetchHealthMatrix } from '@/lib/api/health';
import { formatDateTime } from '@/lib/formatDateTime';
import { formatMoneyMinor } from '@/lib/i18n/labels';
import { useI18n } from '@/lib/i18n/context';
import { probeStatusTone, tenantStatusTone } from '@/lib/platform-tokens';

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

  if (summary.isLoading) {
    return <LoadingSpinner />;
  }

  const data = summary.data;

  return (
    <div className="space-y-6">
      <PageHeader title={t('dashboard.title')} />
      <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-5">
        <KpiCard label={t('dashboard.activeTenants')} value={data?.active ?? 0} />
        <KpiCard label={t('dashboard.trialTenants')} value={data?.trial ?? 0} />
        <KpiCard label={t('dashboard.pastDue')} value={data?.pastDue ?? 0} />
        <KpiCard label={t('dashboard.suspended')} value={data?.suspended ?? 0} />
        <KpiCard
          label={t('dashboard.mrr')}
          value={data ? formatMoneyMinor(data.mrrMinor, data.mrrCurrency) : '—'}
        />
      </div>

      <div className="flex flex-wrap gap-2">
        <Link to="/tenants/new">
          <Button>{t('dashboard.createTenant')}</Button>
        </Link>
        <Link to="/health">
          <Button variant="secondary">{t('dashboard.viewHealth')}</Button>
        </Link>
      </div>

      <div className="grid gap-6 lg:grid-cols-2">
        <Card className="p-4">
          <h2 className="mb-4 text-sm font-semibold">{t('dashboard.healthMatrix')}</h2>
          {health.isLoading ? <LoadingSpinner /> : null}
          {health.data ? (
            <ul className="space-y-2">
              {Object.entries(health.data.probes).map(([name, probe]) => (
                <li key={name} className="flex items-center gap-3 text-sm">
                  <span className={`size-2.5 rounded-full ${probeStatusTone(probe.status)}`} />
                  <span className="flex-1 font-medium">{name}</span>
                  <span className="text-muted-foreground">{probe.status}</span>
                </li>
              ))}
            </ul>
          ) : null}
        </Card>

        <Card className="p-4">
          <div className="mb-4 flex items-center justify-between">
            <h2 className="text-sm font-semibold">{t('dashboard.recentFraud')}</h2>
            <Link to="/fraud" className="text-sm text-primary underline-offset-2 hover:underline">
              {t('common.viewAll')}
            </Link>
          </div>
          {fraud.isLoading ? <LoadingSpinner /> : null}
          {fraud.data?.data.length ? (
            <ul className="divide-y divide-hairline">
              {fraud.data.data.map((event) => (
                <li key={event.id} className="py-2 text-sm">
                  <p className="font-medium">{event.eventType}</p>
                  <p className="text-muted-foreground">
                    {event.severity} · {formatDateTime(event.createdAt)}
                  </p>
                </li>
              ))}
            </ul>
          ) : (
            <EmptyState title={t('common.noResults')} />
          )}
        </Card>
      </div>
    </div>
  );
}

function KpiCard({ label, value }: { label: string; value: number | string }) {
  return (
    <Card className="p-4">
      <p className="text-xs uppercase tracking-wide text-muted-foreground">{label}</p>
      <p
        className={`mt-2 text-2xl font-semibold ${typeof value === 'number' ? tenantStatusTone('Active') : ''}`}
      >
        {value}
      </p>
    </Card>
  );
}
