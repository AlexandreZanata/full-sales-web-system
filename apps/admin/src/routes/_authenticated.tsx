import { Outlet, createFileRoute, redirect } from '@tanstack/react-router';

import { AdminShell } from '@/components/AdminShell';

export const Route = createFileRoute('/_authenticated')({
  beforeLoad: async ({ context }) => {
    const user = await context.auth.ensureSession();
    if (!user) {
      // TanStack Router redirect helper — not a standard Error
      // eslint-disable-next-line @typescript-eslint/only-throw-error
      throw redirect({ to: '/login' });
    }
  },
  component: AuthenticatedLayout,
});

function AuthenticatedLayout() {
  return (
    <AdminShell>
      <Outlet />
    </AdminShell>
  );
}
