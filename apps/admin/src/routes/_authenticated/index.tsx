import { createFileRoute } from '@tanstack/react-router';

import { PageHeader } from '@/components/ui/PageHeader';

export const Route = createFileRoute('/_authenticated/')({
  component: DashboardPage,
});

function DashboardPage() {
  return <PageHeader title="Dashboard" description="Home stats — Phase 29." />;
}
