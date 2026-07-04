import { createFileRoute } from '@tanstack/react-router';

import { PageHeader } from '@/components/ui/PageHeader';

export const Route = createFileRoute('/_authenticated/sales/')({
  component: SalesPage,
});

function SalesPage() {
  return <PageHeader title="Sales" description="Sales admin view — Phase 34." />;
}
