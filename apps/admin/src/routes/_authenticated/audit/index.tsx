import { createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { JsonBlock } from '@/components/ui/JsonBlock';
import { Button } from '@/components/ui/Button';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { useI18n } from '@/lib/i18n/context';
import { fetchAuditEvents } from '@/lib/api/audit';
import { fetchUsers } from '@/lib/api/users';
import type { AuditEvent } from '@/lib/api/types';
import { formatDateTime } from '@/lib/formatDateTime';
import { formatTablePaginationSummary, paginatedResponseToTable } from '@/lib/tablePagination';
import { cn } from '@/lib/utils';

export const Route = createFileRoute('/_authenticated/audit/')({
  component: AuditPage,
});

function AuditPage() {
  const { t } = useI18n();
  const [page, setPage] = useState(1);
  const [expandedIds, setExpandedIds] = useState<Set<string>>(() => new Set());
  const pageSize = 20;

  const events = useQuery({
    queryKey: ['audit', 'events', page, pageSize],
    queryFn: () => fetchAuditEvents({ page, pageSize }),
  });

  const actors = useQuery({
    queryKey: ['users', 'audit-actors'],
    queryFn: () => fetchUsers({ page: 1, pageSize: 50 }),
  });

  const actorNames = useMemo(() => {
    const map = new Map<string, string>();
    for (const user of actors.data?.items ?? []) {
      map.set(user.id, user.name);
    }
    return map;
  }, [actors.data]);

  const pagination = events.data ? paginatedResponseToTable(events.data) : null;

  function toggleExpanded(id: string) {
    setExpandedIds((current) => {
      const next = new Set(current);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  }

  return (
    <div>
      <PageHeader
        title="Audit log"
        description="Append-only audit trail for sensitive actions (RN-PAG3)."
      />

      {events.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : events.data && events.data.items.length > 0 ? (
        <div className="overflow-hidden rounded-lg border border-hairline bg-surface">
          <div className="overflow-x-auto">
            <table className="min-w-full text-sm">
              <caption className="sr-only">Audit events</caption>
              <thead className="border-b border-hairline bg-surface-muted">
                <tr>
                  <th className="px-4 py-3 text-left font-medium text-foreground" scope="col">
                    Time
                  </th>
                  <th className="px-4 py-3 text-left font-medium text-foreground" scope="col">
                    Actor
                  </th>
                  <th className="px-4 py-3 text-left font-medium text-foreground" scope="col">
                    Action
                  </th>
                  <th className="px-4 py-3 text-left font-medium text-foreground" scope="col">
                    Resource
                  </th>
                  <th className="px-4 py-3 text-left font-medium text-foreground" scope="col">
                    Resource ID
                  </th>
                  <th className="px-4 py-3 text-left font-medium text-foreground" scope="col">
                    Details
                  </th>
                </tr>
              </thead>
              <tbody className="bg-surface">
                {events.data.items.map((event, index) => (
                  <AuditEventRows
                    key={event.id}
                    event={event}
                    index={index}
                    actorName={actorNames.get(event.actorId)}
                    expanded={expandedIds.has(event.id)}
                    onToggle={() => {
                      toggleExpanded(event.id);
                    }}
                  />
                ))}
              </tbody>
            </table>
          </div>

          {pagination ? (
            <div
              className="flex flex-col gap-3 border-t border-hairline bg-surface px-4 py-3 sm:flex-row sm:items-center sm:justify-between"
              aria-label="Table pagination"
            >
              <p className="text-sm text-muted-foreground">
                {formatTablePaginationSummary(pagination)}
              </p>
              <div className="flex gap-2">
                <Button
                  type="button"
                  variant="secondary"
                  disabled={pagination.page <= 1}
                  onClick={() => {
                    setPage(pagination.page - 1);
                  }}
                >
                  {t('common.previous')}
                </Button>
                <Button
                  type="button"
                  variant="secondary"
                  disabled={pagination.page >= pagination.totalPages}
                  onClick={() => {
                    setPage(pagination.page + 1);
                  }}
                >
                  {t('common.next')}
                </Button>
              </div>
            </div>
          ) : null}
        </div>
      ) : (
        <EmptyState
          title="No audit events"
          description="Sensitive actions such as payment declarations will appear here."
        />
      )}
    </div>
  );
}

type AuditEventRowsProps = {
  event: AuditEvent;
  index: number;
  actorName?: string;
  expanded: boolean;
  onToggle: () => void;
};

function AuditEventRows({ event, index, actorName, expanded, onToggle }: AuditEventRowsProps) {
  const hasMetadata = event.metadata !== undefined && Object.keys(event.metadata).length > 0;

  return (
    <>
      <tr
        className={cn(
          'border-b border-hairline',
          index % 2 === 0 ? 'bg-surface' : 'bg-surface-muted/60',
        )}
      >
        <td className="px-4 py-3 text-foreground">{formatDateTime(event.createdAt)}</td>
        <td className="px-4 py-3 text-foreground">
          {actorName ?? `${event.actorId.slice(0, 8)}…`}
        </td>
        <td className="px-4 py-3 text-foreground">{event.action}</td>
        <td className="px-4 py-3 text-foreground">{event.resourceType}</td>
        <td className="px-4 py-3 font-mono text-xs text-muted-foreground">
          {event.resourceId.slice(0, 8)}…
        </td>
        <td className="px-4 py-3">
          {hasMetadata ? (
            <Button type="button" variant="secondary" onClick={onToggle}>
              {expanded ? 'Hide metadata' : 'Show metadata'}
            </Button>
          ) : (
            <span className="text-xs text-muted-foreground">—</span>
          )}
        </td>
      </tr>
      {expanded && hasMetadata ? (
        <tr className="border-b border-hairline bg-surface-muted/40">
          <td colSpan={6} className="px-4 py-3">
            <JsonBlock value={event.metadata} defaultOpen />
          </td>
        </tr>
      ) : null}
    </>
  );
}
