import { QueryClient } from '@tanstack/react-query';
import { createRouter } from '@tanstack/react-router';

import type { PlatformAuthContextValue } from '@/auth/usePlatformAuth';
import { routeTree } from './routeTree.gen';

export type RouterContext = {
  auth: PlatformAuthContextValue;
  queryClient: QueryClient;
};

export const router = createRouter({
  routeTree,
  basepath: import.meta.env.BASE_URL.replace(/\/$/, '') || '/',
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
