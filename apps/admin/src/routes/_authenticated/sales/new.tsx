import { createFileRoute, useNavigate } from '@tanstack/react-router';

import { CreateSaleForm } from '@/components/sales/CreateSaleForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { createSale } from '@/lib/api/sales';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/sales/new')({
  component: NewSalePage,
});

function NewSalePage() {
  const navigate = useNavigate();
  const { t } = useI18n();

  return (
    <div>
      <PageHeader
        title={t('sales.create.title')}
        description={t('sales.create.description')}
        back={<PageBackLink label={t('common.backTo.sales')} to="/sales" />}
      />

      <CreateSaleForm
        onSubmit={createSale}
        onSuccess={(sale) => {
          void navigate({ to: '/sales/$id', params: { id: sale.id } });
        }}
      />
    </div>
  );
}
