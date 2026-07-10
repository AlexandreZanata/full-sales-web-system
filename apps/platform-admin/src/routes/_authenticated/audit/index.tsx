import { createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchPlatformAuditEvents } from '@/lib/api/audit';
import type { AuditEvent } from '@/lib/api/types';
import { cursorToTableState } from '@/lib/cursorPagination';
import { formatDateTime } from '@/lib/formatDateTime';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/audit/')({
  component: AuditPage,
});

function AuditPage() {
  const { t } = useI18n();
  const [page, setPage] = useState(1);
  const [cursors, setCursors] = useState<(string | undefined)[]>([undefined]);
  const [action, setAction] = useState('');
  const pageSize = 20;

  const events = useQuery({
    queryKey: ['platform-audit', page, action],
    queryFn: () =>
      fetchPlatformAuditEvents({
        limit: pageSize,
        cursor: cursors[page - 1],
        action: action || undefined,
      }),
  });

  const columns: DataTableColumn<AuditEvent>[] = [
    { id: 'action', header: t('audit.action'), cell: (row) => row.action },
    { id: 'actor', header: t('audit.actor'), cell: (row) => row.actorId },
    { id: 'tenant', header: t('audit.tenant'), cell: (row) => row.tenantId ?? '—' },
    { id: 'created', header: t('common.createdAt'), cell: (row) => formatDateTime(row.createdAt) },
  ];

  const pagination = events.data ? cursorToTableState(page, events.data.pagination.has_more) : null;

  return (
    <div className="space-y-4">
      <PageHeader title={t('audit.title')} />
      <Input
        label={t('audit.action')}
        value={action}
        onChange={(e) => {
          setAction(e.target.value);
          setPage(1);
          setCursors([undefined]);
        }}
      />
      {events.isLoading ? <LoadingSpinner /> : null}
      {events.data?.data.length ? (
        <DataTable
          columns={columns}
          rows={events.data.data}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={(next) => {
            if (next > page && events.data.pagination.next_cursor) {
              setCursors((prev) => [...prev, events.data.pagination.next_cursor ?? undefined]);
            }
            setPage(next);
          }}
        />
      ) : events.isSuccess ? (
        <EmptyState title={t('common.noResults')} />
      ) : null}
    </div>
  );
}
