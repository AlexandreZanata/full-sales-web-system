import { createFileRoute, redirect } from '@tanstack/react-router';

/** Create tenant is a modal on the tenants list — never a full page. */
export const Route = createFileRoute('/_authenticated/tenants/new')({
  beforeLoad: () => {
    // eslint-disable-next-line @typescript-eslint/only-throw-error -- TanStack Router redirect
    throw redirect({
      to: '/tenants',
      search: { modal: 'create' },
    });
  },
});
