import { createFileRoute } from '@tanstack/react-router';

import { PageHeader } from '@/components/ui/PageHeader';

export const Route = createFileRoute('/_authenticated/reports/')({
  component: ReportsPage,
});

function ReportsPage() {
  return <PageHeader title="Reports" description="Signed reports — Phase 35." />;
}
