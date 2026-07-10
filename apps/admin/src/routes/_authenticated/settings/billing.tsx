import { createFileRoute } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import {
  attachPaymentMethod,
  fetchInvoice,
  fetchInvoices,
  fetchSubscription,
} from '@/lib/api/billing';
import type { InvoiceSummary } from '@/lib/api/billing';
import { UPGRADE_PLAN_MAILTO, daysUntil, formatMoneyMinor } from '@/lib/billing/helpers';
import { formatDateTime } from '@/lib/formatDateTime';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

export const Route = createFileRoute('/_authenticated/settings/billing')({
  component: BillingSettingsPage,
});

function BillingSettingsPage() {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [cardToken, setCardToken] = useState('');
  const [pixPayload, setPixPayload] = useState<string | null>(null);

  const subscription = useQuery({
    queryKey: ['billing', 'subscription'],
    queryFn: fetchSubscription,
  });
  const invoices = useQuery({
    queryKey: ['billing', 'invoices'],
    queryFn: () => fetchInvoices({ limit: 50 }),
  });

  const attachCard = useMutation({
    mutationFn: () => attachPaymentMethod(cardToken.trim()),
    onSuccess: () => {
      toast.success('Card saved');
      setCardToken('');
      void queryClient.invalidateQueries({ queryKey: ['billing'] });
    },
  });

  const invoiceColumns: DataTableColumn<InvoiceSummary>[] = [
    { id: 'due', header: t('common.table.date'), cell: (row) => formatDateTime(row.dueDate) },
    {
      id: 'amount',
      header: t('common.table.total'),
      cell: (row) => formatMoneyMinor(row.amountMinor, row.currency),
    },
    { id: 'status', header: t('common.table.status'), cell: (row) => row.status },
    {
      id: 'pdf',
      header: 'PDF',
      cell: (row) => (
        <Button
          variant="secondary"
          className="min-h-8 px-2 text-xs"
          onClick={() => void loadInvoiceExtras(row.id)}
        >
          PDF / PIX
        </Button>
      ),
    },
  ];

  async function loadInvoiceExtras(id: string) {
    const detail = await fetchInvoice(id);
    if (detail.pdfUrl) {
      window.open(detail.pdfUrl, '_blank', 'noopener,noreferrer');
    }
    setPixPayload(t('settings.billing.pixCopy'));
  }

  if (subscription.isLoading) {
    return <LoadingSpinner />;
  }

  const sub = subscription.data;

  return (
    <div className="space-y-4">
      <PageHeader
        title={t('settings.billing.title')}
        description={t('settings.billing.description')}
      />
      {sub ? (
        <Card className="space-y-2 p-4 text-sm">
          <p>
            <strong>{t('settings.billing.plan')}:</strong> {sub.plan.name} ({sub.plan.code})
          </p>
          <p>
            <strong>{t('common.table.status')}:</strong> {sub.status} / {sub.tenantStatus}
          </p>
          {sub.currentPeriodEnd ? (
            <p>
              <strong>{t('settings.billing.renewal')}:</strong>{' '}
              {formatDateTime(sub.currentPeriodEnd)}
            </p>
          ) : null}
          {sub.trialEndsAt ? (
            <p>
              {t('settings.billing.trialEnds').replace(
                '{days}',
                String(daysUntil(sub.trialEndsAt)),
              )}
            </p>
          ) : null}
          <a
            href={UPGRADE_PLAN_MAILTO}
            className="inline-block text-sm text-primary underline-offset-2 hover:underline"
          >
            {t('settings.billing.upgrade')}
          </a>
        </Card>
      ) : null}

      <Card className="space-y-3 p-4">
        <h2 className="text-sm font-semibold">{t('settings.billing.addCard')}</h2>
        <p className="text-xs text-muted-foreground">{t('settings.billing.cardTokenHelp')}</p>
        <form
          className="flex flex-wrap items-end gap-2"
          onSubmit={(event: SubmitEvent) => {
            event.preventDefault();
            attachCard.mutate();
          }}
        >
          <Input
            label={t('settings.billing.cardToken')}
            value={cardToken}
            onChange={(e) => {
              setCardToken(e.target.value);
            }}
          />
          <Button type="submit" disabled={!cardToken.trim() || attachCard.isPending}>
            {t('settings.billing.attachCard')}
          </Button>
        </form>
      </Card>

      <div>
        <h2 className="mb-2 text-sm font-semibold">{t('settings.billing.invoices')}</h2>
        {pixPayload ? <p className="mb-2 text-xs text-muted-foreground">{pixPayload}</p> : null}
        {invoices.data?.data.length ? (
          <DataTable
            columns={invoiceColumns}
            rows={invoices.data.data}
            getRowKey={(row) => row.id}
          />
        ) : (
          <EmptyState title={t('settings.billing.noInvoices')} />
        )}
      </div>
    </div>
  );
}
