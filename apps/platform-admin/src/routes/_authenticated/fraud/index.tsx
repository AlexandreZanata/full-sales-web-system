import { createFileRoute } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { addBlocklistEntry, fetchFraudEvents, resolveFraudEvent } from '@/lib/api/fraud';
import type { FraudEvent } from '@/lib/api/types';
import { formatDateTime } from '@/lib/formatDateTime';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

export const Route = createFileRoute('/_authenticated/fraud/')({
  component: FraudPage,
});

function FraudPage() {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [blockValue, setBlockValue] = useState('');

  const events = useQuery({
    queryKey: ['fraud-events'],
    queryFn: () => fetchFraudEvents({ limit: 50 }),
  });

  const resolve = useMutation({
    mutationFn: (id: string) =>
      resolveFraudEvent(id, { resolution: 'dismissed', note: 'Reviewed in UI' }),
    onSuccess: () => void queryClient.invalidateQueries({ queryKey: ['fraud-events'] }),
  });

  const blocklist = useMutation({
    mutationFn: () =>
      addBlocklistEntry({ kind: 'email', value: blockValue, reason: 'Manual block' }),
    onSuccess: () => {
      toast.success('Blocklist entry added');
      setBlockValue('');
    },
  });

  const columns: DataTableColumn<FraudEvent>[] = [
    { id: 'type', header: 'Type', cell: (row) => row.eventType },
    { id: 'severity', header: 'Severity', cell: (row) => row.severity },
    { id: 'status', header: t('common.status'), cell: (row) => row.status },
    { id: 'created', header: t('common.createdAt'), cell: (row) => formatDateTime(row.createdAt) },
    {
      id: 'actions',
      header: t('common.actions'),
      cell: (row) => (
        <Button
          variant="secondary"
          className="min-h-8 px-2 text-xs"
          onClick={() => {
            resolve.mutate(row.id);
          }}
        >
          {t('fraud.resolve')}
        </Button>
      ),
    },
  ];

  return (
    <div className="space-y-4">
      <PageHeader title={t('fraud.title')} />
      <div className="flex flex-wrap items-end gap-2">
        <Input
          label={t('fraud.addBlocklist')}
          value={blockValue}
          onChange={(e) => {
            setBlockValue(e.target.value);
          }}
        />
        <Button
          onClick={() => {
            blocklist.mutate();
          }}
          disabled={!blockValue.trim()}
        >
          {t('common.save')}
        </Button>
      </div>
      {events.isLoading ? <LoadingSpinner /> : null}
      {events.data?.data.length ? (
        <DataTable columns={columns} rows={events.data.data} getRowKey={(row) => row.id} />
      ) : events.isSuccess ? (
        <EmptyState title={t('common.noResults')} />
      ) : null}
    </div>
  );
}
