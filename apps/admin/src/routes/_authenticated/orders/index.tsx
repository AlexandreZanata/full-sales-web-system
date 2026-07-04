import { createFileRoute } from '@tanstack/react-router';

import { PageHeader } from '@/components/ui/PageHeader';

export const Route = createFileRoute('/_authenticated/orders/')({
  component: OrdersPage,
});

function OrdersPage() {
  return <PageHeader title="Orders" description="Order lifecycle — Phase 33." />;
}
