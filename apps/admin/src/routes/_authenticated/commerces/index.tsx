import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

import { ActiveBadge } from '@/components/users/ActiveBadge';
import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { Select } from '@/components/ui/Select';
import { fetchCommerces } from '@/lib/api/commerces';
import type { CommerceSummary } from '@/lib/api/types';
import { ACTIVE_FILTER_LABELS, type ActiveFilter } from '@/lib/commerces/constants';
import { formatCnpj } from '@/lib/commerces/cnpj';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/commerces/')({
  component: CommercesListPage,
});

const columns: DataTableColumn<CommerceSummary>[] = [
  {
    id: 'cnpj',
    header: 'CNPJ',
    cell: (row) => formatCnpj(row.cnpj),
  },
  {
    id: 'tradeName',
    header: 'Trade name',
    cell: (row) => (
      <Link to="/commerces/$id" params={{ id: row.id }} className="font-medium hover:underline">
        {row.tradeName || row.legalName}
      </Link>
    ),
  },
  {
    id: 'legalName',
    header: 'Legal name',
    cell: (row) => row.legalName,
  },
  {
    id: 'active',
    header: 'Status',
    cell: (row) => <ActiveBadge active={row.active} />,
  },
];

function CommercesListPage() {
  const [page, setPage] = useState(1);
  const [activeFilter, setActiveFilter] = useState<ActiveFilter>('');
  const pageSize = 20;

  const commerces = useQuery({
    queryKey: ['commerces', page, pageSize, activeFilter],
    queryFn: () => fetchCommerces({ page, pageSize, active: activeFilter }),
  });

  const pagination = commerces.data ? paginatedResponseToTable(commerces.data) : null;

  return (
    <div>
      <PageHeader
        title="Commerces"
        description="Register and manage business clients."
        actions={
          <Link to="/commerces/new">
            <Button>Register commerce</Button>
          </Link>
        }
      />

      <div className="mb-4 max-w-xs">
        <Select
          label="Filter by status"
          value={activeFilter}
          onChange={(event) => {
            setActiveFilter(event.target.value as ActiveFilter);
            setPage(1);
          }}
        >
          {(Object.keys(ACTIVE_FILTER_LABELS) as ActiveFilter[]).map((value) => (
            <option key={value || 'all'} value={value}>
              {ACTIVE_FILTER_LABELS[value]}
            </option>
          ))}
        </Select>
      </div>

      {commerces.isLoading ? (
        <div className="flex justify-center py-16">
          <LoadingSpinner />
        </div>
      ) : commerces.data && commerces.data.items.length > 0 ? (
        <DataTable
          caption="Commerces"
          columns={columns}
          rows={commerces.data.items}
          getRowKey={(row) => row.id}
          pagination={pagination}
          onPageChange={setPage}
        />
      ) : (
        <EmptyState
          title="No commerces found"
          description={
            activeFilter
              ? 'Try another status filter or register a new commerce.'
              : 'Register the first commerce to get started.'
          }
        />
      )}
    </div>
  );
}
