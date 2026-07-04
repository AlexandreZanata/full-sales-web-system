import { createFileRoute } from '@tanstack/react-router';

import { PageHeader } from '@/components/ui/PageHeader';

export const Route = createFileRoute('/_authenticated/users/')({
  component: UsersPage,
});

function UsersPage() {
  return <PageHeader title="Users" description="User management — Phase 30." />;
}
