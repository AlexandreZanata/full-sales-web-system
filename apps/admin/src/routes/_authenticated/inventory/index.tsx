import { createFileRoute } from '@tanstack/react-router';

import { PageHeader } from '@/components/ui/PageHeader';

export const Route = createFileRoute('/_authenticated/inventory/')({
  component: InventoryPage,
});

function InventoryPage() {
  return <PageHeader title="Inventory" description="Stock and movements — Phase 32." />;
}
