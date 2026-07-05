import { Link, createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';
import { ClipboardList, SlidersHorizontal } from 'lucide-react';
import { useMemo, useState } from 'react';

import { StockOverviewTable } from '@/components/inventory/StockOverviewTable';
import { Card } from '@/components/ui/Card';
import { EmptyState } from '@/components/ui/EmptyState';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { fetchStockOverview } from '@/lib/api/inventory';
import { useI18n } from '@/lib/i18n/context';
import { paginatedResponseToTable } from '@/lib/tablePagination';

export const Route = createFileRoute('/_authenticated/inventory/')({
  component: InventoryHubPage,
});

function InventoryHubPage() {
  const { t } = useI18n();
  const [page, setPage] = useState(1);
  const [search, setSearch] = useState('');
  const pageSize = 20;

  const links = useMemo(
    () =>
      [
        {
          to: '/inventory/adjustments',
          label: t('inventory.hub.adjustments.title'),
          description: t('inventory.hub.adjustments.description'),
          icon: SlidersHorizontal,
        },
        {
          to: '/inventory/ledger',
          label: t('inventory.hub.ledger.title'),
          description: t('inventory.hub.ledger.description'),
          icon: ClipboardList,
        },
      ] as const,
    [t],
  );

  const overviewQuery = useQuery({
    queryKey: ['inventory', 'balances', page, pageSize, search],
    queryFn: () => fetchStockOverview({ page, pageSize, search: search.trim() || undefined }),
  });

  const pagination = overviewQuery.data ? paginatedResponseToTable(overviewQuery.data) : null;

  return (
    <div className="space-y-8">
      <PageHeader title={t('inventory.hub.title')} description={t('inventory.hub.description')} />

      <div className="grid gap-4 sm:grid-cols-2">
        {links.map((item) => (
          <Link key={item.to} to={item.to} className="block">
            <Card className="h-full transition hover:border-foreground/20">
              <div className="flex items-start gap-4">
                <item.icon className="mt-1 size-5 shrink-0 text-muted-foreground" />
                <div>
                  <p className="font-medium text-foreground">{item.label}</p>
                  <p className="mt-1 text-sm text-muted-foreground">{item.description}</p>
                </div>
              </div>
            </Card>
          </Link>
        ))}
      </div>

      <section className="space-y-4">
        <div className="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
          <div>
            <h2 className="text-lg font-semibold text-foreground">
              {t('inventory.overview.title')}
            </h2>
            <p className="text-sm text-muted-foreground">{t('inventory.overview.description')}</p>
          </div>
          <Input
            label={t('common.search')}
            placeholder={t('inventory.overview.searchPlaceholder')}
            value={search}
            onChange={(event) => {
              setSearch(event.target.value);
              setPage(1);
            }}
            className="sm:max-w-xs"
          />
        </div>

        {overviewQuery.isLoading ? <LoadingSpinner className="py-12" /> : null}

        {overviewQuery.isError ? (
          <EmptyState
            title={t('inventory.overview.loadError')}
            action={
              <button
                type="button"
                className="text-sm font-medium text-foreground underline"
                onClick={() => void overviewQuery.refetch()}
              >
                {t('common.tryAgain')}
              </button>
            }
          />
        ) : null}

        {!overviewQuery.isLoading &&
        !overviewQuery.isError &&
        (overviewQuery.data?.items.length ?? 0) === 0 ? (
          <EmptyState
            title={t('inventory.overview.empty.title')}
            description={t('inventory.overview.empty.description')}
          />
        ) : null}

        {!overviewQuery.isLoading &&
        !overviewQuery.isError &&
        (overviewQuery.data?.items.length ?? 0) > 0 ? (
          <StockOverviewTable
            rows={overviewQuery.data?.items ?? []}
            pagination={pagination}
            onPageChange={setPage}
          />
        ) : null}
      </section>
    </div>
  );
}
