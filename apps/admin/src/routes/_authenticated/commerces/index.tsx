import { createFileRoute } from '@tanstack/react-router';

import { PageHeader } from '@/components/ui/PageHeader';

export const Route = createFileRoute('/_authenticated/commerces/')({
  component: CommercesPage,
});

function CommercesPage() {
  return <PageHeader title="Commerces" description="Commerce registry — Phase 31." />;
}
