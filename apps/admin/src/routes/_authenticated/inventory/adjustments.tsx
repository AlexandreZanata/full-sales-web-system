import { createFileRoute } from '@tanstack/react-router';

import { AdjustmentForm } from '@/components/inventory/AdjustmentForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { useI18n } from '@/lib/i18n/context';

type AdjustmentsSearch = {
  productId?: string;
};

export const Route = createFileRoute('/_authenticated/inventory/adjustments')({
  validateSearch: (search: Record<string, unknown>): AdjustmentsSearch => ({
    productId: typeof search.productId === 'string' ? search.productId : undefined,
  }),
  component: InventoryAdjustmentsPage,
});

function InventoryAdjustmentsPage() {
  const { t } = useI18n();
  const { productId } = Route.useSearch();

  return (
    <div>
      <PageHeader
        title={t('inventory.adjustments.title')}
        description={t('inventory.adjustments.description')}
        back={<PageBackLink label={t('common.backTo.inventory')} to="/inventory" />}
      />

      <AdjustmentForm initialProductId={productId} />
    </div>
  );
}
