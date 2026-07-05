import { Outlet, createFileRoute } from '@tanstack/react-router';

import { PortalShell } from '@/components/PortalShell';

export const Route = createFileRoute('/_authenticated')({
  component: PortalLayout,
});

function PortalLayout() {
  return (
    <PortalShell>
      <Outlet />
    </PortalShell>
  );
}
