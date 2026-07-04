import { Link, Outlet, createRootRouteWithContext, useRouter } from '@tanstack/react-router';

import { Button } from '@/components/ui/Button';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { useI18n } from '@/lib/i18n/context';
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
  const { t } = useI18n();

  return (
    <div className="flex min-h-dvh items-center justify-center bg-background px-4">
      <div className="max-w-md text-center">
        <h1 className="text-7xl font-semibold tracking-tight text-foreground">404</h1>
        <p className="mt-4 text-sm text-muted-foreground">{t('common.pageNotFound')}</p>
        <div className="mt-6">
          <Link to="/">
            <Button>{t('common.backToDashboard')}</Button>
          </Link>
        </div>
      </div>
    </div>
  );
}

function AdminError({ error, reset }: { error: Error; reset: () => void }) {
  const router = useRouter();
  const { t } = useI18n();

  return (
    <div className="flex min-h-dvh items-center justify-center bg-background px-4">
      <div className="max-w-md text-center">
        <h1 className="text-xl font-semibold text-foreground">{t('common.somethingWentWrong')}</h1>
        <p className="mt-2 text-sm text-muted-foreground">
          {error.message || t('common.unexpectedError')}
        </p>
        <div className="mt-6 flex flex-wrap justify-center gap-2">
          <Button
            onClick={() => {
              void router.invalidate();
              reset();
            }}
          >
            {t('common.tryAgain')}
          </Button>
          <Link to="/">
            <Button>{t('common.backToDashboard')}</Button>
          </Link>
        </div>
      </div>
    </div>
  );
}
