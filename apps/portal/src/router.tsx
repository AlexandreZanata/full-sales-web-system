import { QueryClient } from '@tanstack/react-query';
import { createRouter } from '@tanstack/react-router';

import type { PortalAuthContextValue } from '@/auth/usePortalAuth';
import { routeTree } from './routeTree.gen';

export type RouterContext = {
  auth: PortalAuthContextValue;
  queryClient: QueryClient;
};

export const router = createRouter({
  routeTree,
  context: {
    auth: undefined as never,
    queryClient: undefined as never,
  },
});

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router;
  }
}
