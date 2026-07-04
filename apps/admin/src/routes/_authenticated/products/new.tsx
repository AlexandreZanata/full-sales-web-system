import { createFileRoute, useNavigate } from '@tanstack/react-router';

import { CreateProductForm } from '@/components/products/CreateProductForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { createProduct } from '@/lib/api/products';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/products/new')({
  component: NewProductPage,
});

function NewProductPage() {
  const navigate = useNavigate();
  const { t } = useI18n();

  return (
    <div>
      <PageHeader
        title={t('products.create.title')}
        description={t('products.create.description')}
        back={<PageBackLink label={t('common.backTo.products')} to="/products" />}
      />

      <CreateProductForm
        onSubmit={createProduct}
        onSuccess={(product) => {
          void navigate({ to: '/products/$id', params: { id: product.id } });
        }}
      />
    </div>
  );
}
