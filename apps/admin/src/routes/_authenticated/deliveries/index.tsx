import { createFileRoute } from '@tanstack/react-router';

import { PageHeader } from '@/components/ui/PageHeader';

export const Route = createFileRoute('/_authenticated/deliveries/')({
  component: DeliveriesPage,
});

function DeliveriesPage() {
  return <PageHeader title="Deliveries" description="Delivery oversight — Phase 33." />;
}
