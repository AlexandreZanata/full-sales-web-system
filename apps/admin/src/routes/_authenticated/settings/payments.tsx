import { createFileRoute } from '@tanstack/react-router';
import { useQuery } from '@tanstack/react-query';

import { Card } from '@/components/ui/Card';
import { DataTable, type DataTableColumn } from '@/components/ui/DataTable';
import { EmptyState } from '@/components/ui/EmptyState';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageHeader } from '@/components/ui/PageHeader';
import { PaymentSettingsForm } from '@/components/settings/PaymentSettingsForm';
import { fetchSubscription } from '@/lib/api/billing';
import { fetchFraudAlerts, type FraudAlert } from '@/lib/api/fraud';
import {
  fetchPaymentBalance,
  fetchPaymentSettings,
  fetchPaymentTransactions,
  type PaymentTransaction,
} from '@/lib/api/payments';
import { UPGRADE_PLAN_MAILTO, formatMoneyMinor, isStarterPlan } from '@/lib/billing/helpers';
import { formatDateTime } from '@/lib/formatDateTime';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/settings/payments')({
  component: PaymentsSettingsPage,
});

function PaymentsSettingsPage() {
  const { t } = useI18n();

  const subscription = useQuery({
    queryKey: ['billing', 'subscription'],
    queryFn: fetchSubscription,
  });
  const settings = useQuery({ queryKey: ['payments', 'settings'], queryFn: fetchPaymentSettings });
  const balance = useQuery({
    queryKey: ['payments', 'balance'],
    queryFn: fetchPaymentBalance,
    enabled: Boolean(settings.data?.asaas.connected),
  });
  const transactions = useQuery({
    queryKey: ['payments', 'transactions'],
    queryFn: () => fetchPaymentTransactions({ limit: 20 }),
    enabled: Boolean(settings.data?.asaas.connected),
  });
  const fraud = useQuery({ queryKey: ['fraud', 'alerts'], queryFn: () => fetchFraudAlerts(10) });

  const starter = subscription.data ? isStarterPlan(subscription.data.plan.code) : false;

  if (settings.isLoading || subscription.isLoading) {
    return <LoadingSpinner />;
  }
  if (!settings.data) {
    return null;
  }

  if (starter) {
    return (
      <div className="space-y-4">
        <PageHeader
          title={t('settings.payments.title')}
          description={t('settings.payments.starterLocked')}
        />
        <a
          href={UPGRADE_PLAN_MAILTO}
          className="text-sm text-primary underline-offset-2 hover:underline"
        >
          {t('settings.billing.upgrade')}
        </a>
      </div>
    );
  }

  const txColumns: DataTableColumn<PaymentTransaction>[] = [
    { id: 'date', header: t('common.table.date'), cell: (row) => row.date },
    { id: 'type', header: 'Type', cell: (row) => row.type },
    {
      id: 'amount',
      header: t('common.table.total'),
      cell: (row) => formatMoneyMinor(row.amountMinor, 'BRL'),
    },
  ];
  const fraudColumns: DataTableColumn<FraudAlert>[] = [
    { id: 'type', header: 'Type', cell: (row) => row.eventType },
    { id: 'severity', header: 'Severity', cell: (row) => row.severity },
    { id: 'when', header: t('common.table.date'), cell: (row) => formatDateTime(row.createdAt) },
  ];

  return (
    <div className="space-y-4">
      <PageHeader
        title={t('settings.payments.title')}
        description={t('settings.payments.description')}
      />
      <PaymentSettingsForm settings={settings.data} />

      {balance.data ? (
        <Card className="p-4 text-sm">
          {t('settings.payments.balance')}:{' '}
          {formatMoneyMinor(balance.data.balanceMinor, balance.data.currency)}
        </Card>
      ) : null}

      {transactions.data?.data.length ? (
        <div>
          <h2 className="mb-2 text-sm font-semibold">{t('settings.payments.transactions')}</h2>
          <DataTable
            columns={txColumns}
            rows={transactions.data.data}
            getRowKey={(row) => row.id}
          />
        </div>
      ) : null}

      {fraud.data?.length ? (
        <div>
          <h2 className="mb-2 text-sm font-semibold">{t('settings.payments.fraudAlerts')}</h2>
          <DataTable columns={fraudColumns} rows={fraud.data} getRowKey={(row) => row.id} />
        </div>
      ) : (
        <EmptyState title={t('settings.payments.fraudAlerts')} />
      )}
    </div>
  );
}
