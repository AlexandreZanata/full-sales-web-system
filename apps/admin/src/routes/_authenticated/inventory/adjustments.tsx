import { createFileRoute } from '@tanstack/react-router';

import { AdjustmentForm } from '@/components/inventory/AdjustmentForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';

export const Route = createFileRoute('/_authenticated/inventory/adjustments')({
  component: InventoryAdjustmentsPage,
});

function InventoryAdjustmentsPage() {
  return (
    <div>
      <PageHeader
        title="Stock adjustments"
        description="Record manual inventory corrections."
        back={<PageBackLink label="Back to inventory" to="/inventory" />}
      />

      <AdjustmentForm />
    </div>
  );
}
