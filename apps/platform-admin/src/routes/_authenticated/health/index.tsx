import { createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { Card } from '@/components/ui/Card';
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

  const since = useMemo(() => new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(), []);
  const history = useQuery({
    queryKey: ['health-history', probe],
    queryFn: () => fetchHealthHistory({ probe, since }),
    enabled: Boolean(probe),
  });

  return (
    <div className="space-y-6">
      <PageHeader title={t('health.title')} />
      {matrix.isLoading ? <LoadingSpinner /> : null}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {matrix.data
          ? Object.entries(matrix.data.probes).map(([name, entry]) => (
              <Card key={name} className="p-4">
                <div className="flex items-center gap-2">
                  <span className={`size-3 rounded-full ${probeStatusTone(entry.status)}`} />
                  <h3 className="font-medium">{name}</h3>
                </div>
                <p className="mt-2 text-sm text-muted-foreground">{entry.status}</p>
                <p className="text-xs text-muted-foreground">24h uptime: {entry.uptime24hPct}%</p>
              </Card>
            ))
          : null}
      </div>
      <div className="space-y-2">
        <h2 className="text-sm font-semibold">{t('health.history')}</h2>
        <Select
          label="Probe"
          value={probe || probeNames[0] || ''}
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
        {history.data?.points.length ? (
          <ul className="max-h-64 overflow-y-auto rounded border border-hairline text-sm">
            {history.data.points.map((point) => (
              <li
                key={point.checkedAt}
                className="flex justify-between border-b border-hairline px-3 py-2 last:border-0"
              >
                <span>{point.checkedAt}</span>
                <span>{point.status}</span>
              </li>
            ))}
          </ul>
        ) : null}
      </div>
    </div>
  );
}
