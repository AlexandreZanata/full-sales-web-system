import { createFileRoute } from '@tanstack/react-router';

import { AdjustmentForm } from '@/components/inventory/AdjustmentForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/inventory/adjustments')({
  component: InventoryAdjustmentsPage,
});

function InventoryAdjustmentsPage() {
  const { t } = useI18n();

  return (
    <div>
      <PageHeader
        title={t('inventory.adjustments.title')}
        description={t('inventory.adjustments.description')}
        back={<PageBackLink label={t('common.backTo.inventory')} to="/inventory" />}
      />

      <AdjustmentForm />
    </div>
  );
}
