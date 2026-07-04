import { Link, createFileRoute, useNavigate } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

import { ReportTypeBadge } from '@/components/reports/ReportTypeBadge';
import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchReports } from '@/lib/api/reports';
import type { Report } from '@/lib/api/types';
import { formatDateTime } from '@/lib/formatDateTime';
import { formatReportPeriod } from '@/lib/reports/formatPeriod';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/reports/')({
  component: ReportsListPage,
});

function ReportsListPage() {
  const navigate = useNavigate();
  const [page, setPage] = useState(1);
  const pageSize = 20;

  const reports = useQuery({
    queryKey: ['reports', page, pageSize],
    queryFn: () => fetchReports({ page, pageSize }),
  });

  const pagination = reports.data ? paginatedResponseToTable(reports.data) : null;

  const columns: DataTableColumn<Report>[] = [
    {
      id: 'reportType',
      header: 'Type',
      cell: (row) => <ReportTypeBadge reportType={row.reportType} />,
    },
    {
      id: 'period',
      header: 'Period',
      cell: (row) => formatReportPeriod(row.periodStart, row.periodEnd),
    },
    {
      id: 'generatedAt',
      header: 'Generated',
      cell: (row) => formatDateTime(row.generatedAt),
    },
    {
      id: 'publicKeyId',
      header: 'Key',
      cell: (row) => (
        <span className="font-mono text-xs text-muted-foreground">{row.publicKeyId}</span>
      ),
    },
  ];

  return (
    <div>
      <PageHeader
        title="Reports"
        description="Signed settlement reports with Ed25519 integrity proof."
        actions={
          <Link to="/reports/new">
            <Button>Generate report</Button>
          </Link>
        }
      />

      {reports.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : reports.data && reports.data.items.length > 0 ? (
        <DataTable
          caption="Reports"
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
          title="No reports yet"
          description="Generate a signed report for a driver and period."
        />
      )}
    </div>
  );
}
