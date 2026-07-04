import { createFileRoute } from '@tanstack/react-router';

import { PageHeader } from '@/components/ui/PageHeader';

export const Route = createFileRoute('/_authenticated/audit/')({
  component: AuditPage,
});

function AuditPage() {
  return <PageHeader title="Audit" description="Audit event log — Phase 35." />;
}
