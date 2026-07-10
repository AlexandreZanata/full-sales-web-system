import { createFileRoute } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';

import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchPlatformDomains, forceVerifyDomain } from '@/lib/api/domains';
import type { PlatformDomain } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/domains/')({
  component: DomainsPage,
});

function DomainsPage() {
  const { t } = useI18n();
  const queryClient = useQueryClient();

  const domains = useQuery({
    queryKey: ['platform-domains'],
    queryFn: () => fetchPlatformDomains({ limit: 100 }),
  });

  const verify = useMutation({
    mutationFn: forceVerifyDomain,
    onSuccess: () => void queryClient.invalidateQueries({ queryKey: ['platform-domains'] }),
  });

  const columns: DataTableColumn<PlatformDomain>[] = [
    { id: 'host', header: 'Hostname', cell: (row) => row.hostname },
    { id: 'tenant', header: 'Tenant', cell: (row) => row.tenantId },
    { id: 'status', header: t('common.status'), cell: (row) => row.status },
    {
      id: 'actions',
      header: t('common.actions'),
      cell: (row) => (
        <Button
          variant="secondary"
          className="min-h-8 px-2 text-xs"
          onClick={() => {
            verify.mutate(row.id);
          }}
        >
          {t('domains.forceVerify')}
        </Button>
      ),
    },
  ];

  return (
    <div className="space-y-4">
      <PageHeader title={t('domains.title')} />
      {domains.isLoading ? <LoadingSpinner /> : null}
      {domains.data?.data.length ? (
        <DataTable columns={columns} rows={domains.data.data} getRowKey={(row) => row.id} />
      ) : domains.isSuccess ? (
        <EmptyState title={t('common.noResults')} />
      ) : null}
    </div>
  );
}
