import { Link, Outlet, createRootRouteWithContext, useRouter } from '@tanstack/react-router';

import { Button } from '@/components/ui/Button';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import type { RouterContext } from '@/router';

export const Route = createRootRouteWithContext<RouterContext>()({
  component: RootComponent,
  pendingComponent: PendingRoot,
  notFoundComponent: AdminNotFound,
  errorComponent: AdminError,
});

function RootComponent() {
  return <Outlet />;
}

function PendingRoot() {
  return (
    <div className="flex min-h-dvh items-center justify-center bg-background">
      <LoadingSpinner />
    </div>
  );
}

function AdminNotFound() {
  return (
    <div className="flex min-h-dvh items-center justify-center bg-background px-4">
      <div className="max-w-md text-center">
        <h1 className="text-7xl font-semibold tracking-tight text-foreground">404</h1>
        <p className="mt-4 text-sm text-muted-foreground">This admin page does not exist.</p>
        <div className="mt-6">
          <Link to="/">
            <Button>Back to dashboard</Button>
          </Link>
        </div>
      </div>
    </div>
  );
}

function AdminError({ error, reset }: { error: Error; reset: () => void }) {
  const router = useRouter();

  return (
    <div className="flex min-h-dvh items-center justify-center bg-background px-4">
      <div className="max-w-md text-center">
        <h1 className="text-xl font-semibold text-foreground">Something went wrong</h1>
        <p className="mt-2 text-sm text-muted-foreground">
          {error.message || 'An unexpected error occurred while loading this page.'}
        </p>
        <div className="mt-6 flex flex-wrap justify-center gap-2">
          <Button
            onClick={() => {
              void router.invalidate();
              reset();
            }}
          >
            Try again
          </Button>
          <Link to="/">
            <Button>Back to dashboard</Button>
          </Link>
        </div>
      </div>
    </div>
  );
}
