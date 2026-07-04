import { Outlet, createFileRoute, redirect } from '@tanstack/react-router';

import { FieldShell } from '@/components/FieldShell';

export const Route = createFileRoute('/_authenticated')({
  beforeLoad: async ({ context }) => {
    const user = await context.auth.ensureSession();
    if (!user) {
      // eslint-disable-next-line @typescript-eslint/only-throw-error
      throw redirect({ to: '/login' });
    }
  },
  component: () => (
    <FieldShell>
      <Outlet />
    </FieldShell>
  ),
});
