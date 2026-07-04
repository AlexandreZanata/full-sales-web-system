import { createFileRoute } from '@tanstack/react-router';

import { PageHeader } from '@/components/ui/PageHeader';

export const Route = createFileRoute('/_authenticated/products/')({
  component: ProductsPage,
});

function ProductsPage() {
  return <PageHeader title="Products" description="Product catalog — Phase 32." />;
}
