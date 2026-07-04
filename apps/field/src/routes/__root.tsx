import { Link, Outlet, createRootRouteWithContext, useRouter } from '@tanstack/react-router';

import { Button } from '@/components/ui/Button';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { useI18n } from '@/lib/i18n/context';
import type { RouterContext } from '@/router';

export const Route = createRootRouteWithContext<RouterContext>()({
  component: () => <Outlet />,
  pendingComponent: () => (
    <div className="flex min-h-dvh items-center justify-center">
      <LoadingSpinner />
    </div>
  ),
  notFoundComponent: NotFound,
  errorComponent: RootError,
});

function NotFound() {
  const { t } = useI18n();
  return (
    <div className="flex min-h-dvh items-center justify-center px-4">
      <div className="text-center">
        <h1 className="text-7xl font-semibold">404</h1>
        <p className="mt-2 text-sm text-muted-foreground">{t('common.pageNotFound')}</p>
        <Link to="/" className="mt-4 inline-block">
          <Button>{t('nav.sales')}</Button>
        </Link>
      </div>
    </div>
  );
}

function RootError({ error, reset }: { error: Error; reset: () => void }) {
  const router = useRouter();
  const { t } = useI18n();
  return (
    <div className="flex min-h-dvh items-center justify-center px-4">
      <div className="text-center">
        <h1 className="text-xl font-semibold">{t('common.somethingWentWrong')}</h1>
        <p className="mt-2 text-sm text-muted-foreground">{error.message}</p>
        <Button
          className="mt-4"
          onClick={() => {
            void router.invalidate();
            reset();
          }}
        >
          {t('common.tryAgain')}
        </Button>
      </div>
    </div>
  );
}
