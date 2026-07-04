import { Link, useRouterState } from '@tanstack/react-router';
import { LogOut, Plus, Receipt } from 'lucide-react';
import { type ReactNode } from 'react';

import { useFieldAuth } from '@/auth/useFieldAuth';
import { LocaleSwitcher } from '@/components/LocaleSwitcher';
import { Button } from '@/components/ui/Button';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type FieldShellProps = { children: ReactNode };

const NAV = [
  { to: '/', labelKey: 'nav.sales' as const, icon: Receipt },
  { to: '/sales/new', labelKey: 'nav.newSale' as const, icon: Plus },
];

export function FieldShell({ children }: FieldShellProps) {
  const { t } = useI18n();
  const { logout, user } = useFieldAuth();
  const pathname = useRouterState({ select: (s) => s.location.pathname });

  return (
    <div className="flex min-h-dvh flex-col bg-background">
      <header className="sticky top-0 z-20 border-b border-hairline bg-surface/95 backdrop-blur">
        <div className="mx-auto flex h-14 max-w-3xl items-center justify-between gap-3 px-4">
          <div className="flex items-center gap-2">
            <span className="text-sm font-semibold">{t('auth.fieldLabel')}</span>
            <nav className="hidden gap-1 md:flex">
              {NAV.map(({ to, labelKey, icon: Icon }) => (
                <Link
                  key={to}
                  to={to}
                  className={cn(
                    'inline-flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium',
                    pathname === to
                      ? 'bg-primary text-primary-foreground'
                      : 'text-muted-foreground hover:bg-surface-muted',
                  )}
                >
                  <Icon className="size-4" aria-hidden />
                  {t(labelKey)}
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
              <LogOut className="size-4" />
            </Button>
          </div>
        </div>
      </header>

      <main className="mx-auto w-full max-w-3xl flex-1 px-4 py-4 pb-24 md:pb-6">{children}</main>

      <nav className="fixed inset-x-0 bottom-0 border-t border-hairline bg-surface md:hidden">
        <div className="mx-auto grid max-w-3xl grid-cols-2">
          {NAV.map(({ to, labelKey, icon: Icon }) => (
            <Link
              key={to}
              to={to}
              className={cn(
                'flex flex-col items-center gap-1 py-2 text-xs font-medium',
                pathname === to ? 'text-foreground' : 'text-muted-foreground',
              )}
            >
              <Icon className="size-5" aria-hidden />
              {t(labelKey)}
            </Link>
          ))}
        </div>
      </nav>
    </div>
  );
}
