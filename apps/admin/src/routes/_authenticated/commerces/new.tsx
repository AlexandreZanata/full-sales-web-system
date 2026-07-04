import { createFileRoute, useNavigate } from '@tanstack/react-router';

import { CreateCommerceForm } from '@/components/commerces/CreateCommerceForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { createCommerce } from '@/lib/api/commerces';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/commerces/new')({
  component: NewCommercePage,
});

function NewCommercePage() {
  const navigate = useNavigate();
  const { t } = useI18n();

  return (
    <div>
      <PageHeader
        title={t('commerces.create.title')}
        description={t('commerces.create.description')}
        back={<PageBackLink label={t('common.backTo.commerces')} to="/commerces" />}
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
