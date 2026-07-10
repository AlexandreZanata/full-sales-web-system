import { createFileRoute } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';

import { Button } from '@/components/ui/Button';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchTenants, runDunningJob } from '@/lib/api/tenants';
import type { TenantListItem } from '@/lib/api/types';
import { tenantStatusTone } from '@/lib/platform-tokens';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

export const Route = createFileRoute('/_authenticated/billing/')({
  component: BillingPage,
});

function BillingPage() {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();

  const tenants = useQuery({
    queryKey: ['billing-tenants'],
    queryFn: () => fetchTenants({ limit: 100 }),
  });

  const dunning = useMutation({
    mutationFn: runDunningJob,
    onSuccess: (result) => {
      toast.success(`Dunning processed ${String(result.processed.length)} tenants`);
      void queryClient.invalidateQueries({ queryKey: ['billing-tenants'] });
    },
  });

  const columns: DataTableColumn<TenantListItem>[] = [
    { id: 'name', header: t('common.name'), cell: (row) => row.displayName },
    {
      id: 'status',
      header: t('common.status'),
      cell: (row) => <span className={tenantStatusTone(row.status)}>{row.status}</span>,
    },
    { id: 'plan', header: 'Plan', cell: (row) => row.planId ?? '—' },
  ];

  const billingRows =
    tenants.data?.data.filter((row) =>
      ['Active', 'Trial', 'PastDue', 'Suspended'].includes(row.status),
    ) ?? [];

  return (
    <div className="space-y-4">
      <PageHeader
        title={t('billing.title')}
        description={t('billing.description')}
        actions={
          <Button
            onClick={() => {
              dunning.mutate();
            }}
            disabled={dunning.isPending}
          >
            {t('billing.runDunning')}
          </Button>
        }
      />
      {tenants.isLoading ? <LoadingSpinner /> : null}
      {billingRows.length ? (
        <DataTable columns={columns} rows={billingRows} getRowKey={(row) => row.id} />
      ) : tenants.isSuccess ? (
        <EmptyState title={t('common.noResults')} />
      ) : null}
    </div>
  );
}
