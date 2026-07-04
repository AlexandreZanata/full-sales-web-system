import { Link, useRouterState } from '@tanstack/react-router';
import { LayoutGrid, LogOut, Package, ShoppingCart } from 'lucide-react';
import { type ReactNode } from 'react';

import { usePortalAuth } from '@/auth/usePortalAuth';
import { useCart } from '@/cart/CartProvider';
import { LocaleSwitcher } from '@/components/LocaleSwitcher';
import { Button } from '@/components/ui/Button';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type PortalShellProps = {
  children: ReactNode;
};

const NAV_ITEMS = [
  { to: '/', labelKey: 'nav.catalog' as const, icon: LayoutGrid },
  { to: '/cart', labelKey: 'nav.cart' as const, icon: ShoppingCart },
  { to: '/orders', labelKey: 'nav.orders' as const, icon: Package },
];

export function PortalShell({ children }: PortalShellProps) {
  const { t } = useI18n();
  const { logout, user } = usePortalAuth();
  const { itemCount } = useCart();
  const pathname = useRouterState({ select: (state) => state.location.pathname });

  return (
    <div className="flex min-h-dvh flex-col bg-background">
      <header className="sticky top-0 z-20 border-b border-hairline bg-surface/95 backdrop-blur">
        <div className="mx-auto flex h-14 max-w-6xl items-center justify-between gap-4 px-4">
          <div className="flex items-center gap-3">
            <span className="text-sm font-semibold tracking-tight text-foreground">
              {t('auth.portalLabel')}
            </span>
            <nav className="hidden items-center gap-1 md:flex" aria-label="Main">
              {NAV_ITEMS.map(({ to, labelKey, icon: Icon }) => (
                <Link
                  key={to}
                  to={to}
                  className={cn(
                    'inline-flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium transition-colors',
                    pathname === to || (to !== '/' && pathname.startsWith(to))
                      ? 'bg-primary text-primary-foreground'
                      : 'text-muted-foreground hover:bg-surface-muted hover:text-foreground',
                  )}
                >
                  <Icon className="size-4" aria-hidden />
                  {t(labelKey)}
                  {to === '/cart' && itemCount > 0 ? (
                    <span className="rounded-full bg-accent px-1.5 text-xs text-primary-foreground">
                      {itemCount}
                    </span>
                  ) : null}
                </Link>
              ))}
            </nav>
          </div>
          <div className="flex items-center gap-2">
            <LocaleSwitcher />
            {user ? (
              <span className="hidden text-xs text-muted-foreground sm:inline">{user.email}</span>
            ) : null}
            <Button variant="ghost" className="h-9 px-2" onClick={() => void logout()}>
              <LogOut className="size-4" aria-hidden />
              <span className="sr-only md:not-sr-only md:ml-2">{t('auth.logout')}</span>
            </Button>
          </div>
        </div>
      </header>

      <main className="mx-auto w-full max-w-6xl flex-1 px-4 py-4 pb-24 md:pb-6">{children}</main>

      <nav
        className="fixed inset-x-0 bottom-0 z-20 border-t border-hairline bg-surface md:hidden"
        aria-label="Mobile"
      >
        <div className="mx-auto grid max-w-6xl grid-cols-3">
          {NAV_ITEMS.map(({ to, labelKey, icon: Icon }) => (
            <Link
              key={to}
              to={to}
              className={cn(
                'flex flex-col items-center gap-1 py-2 text-xs font-medium',
                pathname === to || (to !== '/' && pathname.startsWith(to))
                  ? 'text-foreground'
                  : 'text-muted-foreground',
              )}
            >
              <span className="relative">
                <Icon className="size-5" aria-hidden />
                {to === '/cart' && itemCount > 0 ? (
                  <span className="absolute -right-2 -top-1 rounded-full bg-accent px-1 text-[10px] text-primary-foreground">
                    {itemCount}
                  </span>
                ) : null}
              </span>
              {t(labelKey)}
            </Link>
          ))}
        </div>
      </nav>
    </div>
  );
}
