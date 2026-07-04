import { createFileRoute, useNavigate } from '@tanstack/react-router';

import { CreateProductForm } from '@/components/products/CreateProductForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { createProduct } from '@/lib/api/products';

export const Route = createFileRoute('/_authenticated/products/new')({
  component: NewProductPage,
});

function NewProductPage() {
  const navigate = useNavigate();

  return (
    <div>
      <PageHeader
        title="New product"
        description="Add a SKU to the catalog."
        back={<PageBackLink label="Back to products" to="/products" />}
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
