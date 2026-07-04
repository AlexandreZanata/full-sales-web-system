import { createFileRoute, useNavigate } from '@tanstack/react-router';

import { CreateSaleForm } from '@/components/sales/CreateSaleForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { createSale } from '@/lib/api/sales';

export const Route = createFileRoute('/_authenticated/sales/new')({
  component: NewSalePage,
});

function NewSalePage() {
  const navigate = useNavigate();

  return (
    <div>
      <PageHeader
        title="New sale"
        description="Record a field sale on behalf of the tenant."
        back={<PageBackLink label="Back to sales" to="/sales" />}
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
