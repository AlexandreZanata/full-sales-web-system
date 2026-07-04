import { Link, createFileRoute, useNavigate } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { ReportTypeBadge } from '@/components/reports/ReportTypeBadge';
import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchReports } from '@/lib/api/reports';
import type { Report } from '@/lib/api/types';
import { formatDateTime } from '@/lib/formatDateTime';
import { useI18n } from '@/lib/i18n/context';
import { formatReportPeriod } from '@/lib/reports/formatPeriod';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/reports/')({
  component: ReportsListPage,
});

function ReportsListPage() {
  const { t } = useI18n();
  const navigate = useNavigate();
  const [page, setPage] = useState(1);
  const pageSize = 20;

  const reports = useQuery({
    queryKey: ['reports', page, pageSize],
    queryFn: () => fetchReports({ page, pageSize }),
  });

  const pagination = reports.data ? paginatedResponseToTable(reports.data) : null;

  const columns: DataTableColumn<Report>[] = useMemo(
    () => [
      {
        id: 'reportType',
        header: t('forms.fields.type'),
        cell: (row) => <ReportTypeBadge reportType={row.reportType} />,
      },
      {
        id: 'period',
        header: t('forms.fields.period'),
        cell: (row) => formatReportPeriod(row.periodStart, row.periodEnd),
      },
      {
        id: 'generatedAt',
        header: t('forms.fields.generated'),
        cell: (row) => formatDateTime(row.generatedAt),
      },
      {
        id: 'publicKeyId',
        header: t('forms.fields.publicKey'),
        cell: (row) => (
          <span className="font-mono text-xs text-muted-foreground">{row.publicKeyId}</span>
        ),
      },
    ],
    [t],
  );

  return (
    <div>
      <PageHeader
        title={t('reports.list.title')}
        description={t('reports.list.description')}
        actions={
          <Link to="/reports/new">
            <Button>{t('reports.list.generate')}</Button>
          </Link>
        }
      />

      {reports.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : reports.data && reports.data.items.length > 0 ? (
        <DataTable
          caption={t('reports.list.caption')}
          columns={columns}
          rows={reports.data.items}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={setPage}
          onRowClick={(row) => {
            void navigate({ to: '/reports/$id', params: { id: row.id } });
          }}
        />
      ) : (
        <EmptyState
          title={t('reports.list.empty.title')}
          description={t('reports.list.empty.description')}
        />
      )}
    </div>
  );
}
