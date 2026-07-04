import { createFileRoute, useNavigate } from '@tanstack/react-router';

import { CreateCommerceForm } from '@/components/commerces/CreateCommerceForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { createCommerce } from '@/lib/api/commerces';

export const Route = createFileRoute('/_authenticated/commerces/new')({
  component: NewCommercePage,
});

function NewCommercePage() {
  const navigate = useNavigate();

  return (
    <div>
      <PageHeader
        title="Register commerce"
        description="Create a new business client with CNPJ and address."
        back={<PageBackLink label="Back to commerces" to="/commerces" />}
      />

      <CreateCommerceForm
        onSubmit={createCommerce}
        onSuccess={(commerce) => {
          void navigate({ to: '/commerces/$id', params: { id: commerce.id } });
        }}
      />
    </div>
  );
}
