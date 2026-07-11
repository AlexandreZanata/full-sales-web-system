import { createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useEffect, useMemo, useState } from 'react';

import { HealthHistoryChart, mapHealthHistoryPoints } from '@/components/charts/HealthHistoryChart';
import { HealthUptimeChart } from '@/components/charts/HealthUptimeChart';
import { Card } from '@/components/ui/Card';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchHealthHistory, fetchHealthMatrix } from '@/lib/api/health';
import { useI18n } from '@/lib/i18n/context';
import { probeStatusTone } from '@/lib/platform-tokens';

export const Route = createFileRoute('/_authenticated/health/')({
  component: HealthPage,
});

function HealthPage() {
  const { t } = useI18n();
  const matrix = useQuery({ queryKey: ['health-matrix'], queryFn: fetchHealthMatrix });
  const probeNames = useMemo(() => Object.keys(matrix.data?.probes ?? {}), [matrix.data]);
  const [probe, setProbe] = useState('');

  useEffect(() => {
    if (!probe && probeNames[0]) {
      setProbe(probeNames[0]);
    }
  }, [probe, probeNames]);

  const since = useMemo(() => new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(), []);
  const history = useQuery({
    queryKey: ['health-history', probe],
    queryFn: () => fetchHealthHistory({ probe, since }),
    enabled: Boolean(probe),
  });

  const uptimeRows = useMemo(() => {
    if (!matrix.data) return [];
    return Object.entries(matrix.data.probes).map(([name, entry]) => ({
      name,
      uptime: entry.uptime24hPct,
      status: entry.status,
    }));
  }, [matrix.data]);

  const historyRows = useMemo(
    () => mapHealthHistoryPoints(history.data?.points ?? []),
    [history.data],
  );

  return (
    <div className="space-y-6">
      <PageHeader title={t('health.title')} description={t('health.subtitle')} />
      {matrix.isLoading ? <LoadingSpinner /> : null}

      {uptimeRows.length ? (
        <Card className="p-5">
          <h2 className="text-sm font-semibold">{t('dashboard.healthUptime')}</h2>
          <HealthUptimeChart rows={uptimeRows} uptimeLabel={t('dashboard.uptime24h')} />
        </Card>
      ) : null}

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {matrix.data
          ? Object.entries(matrix.data.probes).map(([name, entry]) => (
              <Card key={name} className="p-4">
                <div className="flex items-center gap-2">
                  <span className={`size-3 rounded-full ${probeStatusTone(entry.status)}`} />
                  <h3 className="font-medium">{name}</h3>
                </div>
                <p className="mt-2 text-sm capitalize text-muted-foreground">{entry.status}</p>
                <p className="text-xs text-muted-foreground">
                  {t('dashboard.uptime24h')}: {entry.uptime24hPct}%
                </p>
              </Card>
            ))
          : null}
      </div>

      <Card className="space-y-4 p-5">
        <div>
          <h2 className="text-sm font-semibold">{t('health.history')}</h2>
          <p className="mt-1 text-xs text-muted-foreground">{t('health.historyHint')}</p>
        </div>
        <Select
          label="Probe"
          value={probe}
          onChange={(e) => {
            setProbe(e.target.value);
          }}
        >
          {probeNames.map((name) => (
            <option key={name} value={name}>
              {name}
            </option>
          ))}
        </Select>
        {history.isLoading ? <LoadingSpinner /> : null}
        {historyRows.length ? (
          <HealthHistoryChart rows={historyRows} uptimeLabel={t('dashboard.uptime24h')} />
        ) : (
          <EmptyState title={t('common.noResults')} />
        )}
      </Card>
    </div>
  );
}
