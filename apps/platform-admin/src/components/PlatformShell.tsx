import { useRouterState } from '@tanstack/react-router';
import { LogOut, Menu, X } from 'lucide-react';
import { useEffect, useState, type ReactNode } from 'react';

import { usePlatformAuth } from '@/auth/usePlatformAuth';
import { ImpersonationBanner } from '@/components/ImpersonationBanner';
import { LocaleSwitcher } from '@/components/LocaleSwitcher';
import { PlatformBrand, PlatformNavLinks } from '@/components/PlatformNavLinks';
import { Button } from '@/components/ui/Button';
import { useI18n } from '@/lib/i18n/context';
import { platformTokens } from '@/lib/platform-tokens';
import { cn } from '@/lib/utils';

type PlatformShellProps = {
  children: ReactNode;
};

export function PlatformShell({ children }: PlatformShellProps) {
  const { user, logout } = usePlatformAuth();
  const { t } = useI18n();
  const [mobileNavOpen, setMobileNavOpen] = useState(false);
  const pathname = useRouterState({ select: (state) => state.location.pathname });

  useEffect(() => {
    setMobileNavOpen(false);
  }, [pathname]);

  async function handleLogout() {
    await logout();
    window.location.assign('/login');
  }

  return (
    <div className="flex min-h-dvh flex-col bg-background">
      <ImpersonationBanner />
      <div className="flex min-h-0 flex-1 flex-col md:grid md:grid-cols-[15rem_1fr] md:grid-rows-[6.25rem_minmax(0,1fr)]">
      <header className={cn(platformTokens.shellHeaderBar, 'md:hidden')}>
        <Button
          type="button"
          variant="ghost"
          className="min-h-10 min-w-10"
          aria-expanded={mobileNavOpen}
          aria-label={t('shell.openNav')}
          onClick={() => {
            setMobileNavOpen(true);
          }}
        >
          <Menu className="size-5" />
        </Button>
        <span className="flex-1 truncate text-sm font-medium">{user?.email}</span>
        <Button
          variant="ghost"
          className="min-h-10 min-w-10"
          aria-label={t('auth.logout')}
          onClick={() => void handleLogout()}
        >
          <LogOut className="size-4" />
        </Button>
      </header>

      <PlatformBrand className="col-start-1 row-start-1 hidden md:flex" />
      <header
        className={cn('col-start-2 row-start-1 hidden md:flex', platformTokens.shellHeaderBar)}
      >
        <LocaleSwitcher className="mr-auto" />
        <span className="text-sm text-muted-foreground">{user?.email}</span>
        <Button variant="ghost" onClick={() => void handleLogout()}>
          {t('auth.logout')}
        </Button>
      </header>

      <aside className="col-start-1 row-start-2 hidden border-r border-hairline bg-surface-muted md:block">
        <PlatformNavLinks pathname={pathname} />
      </aside>

      {mobileNavOpen ? (
        <div className="fixed inset-0 z-50 md:hidden">
          <button
            type="button"
            className="absolute inset-0 bg-black/40"
            aria-label={t('shell.closeNav')}
            onClick={() => {
              setMobileNavOpen(false);
            }}
          />
          <div className="relative flex h-full w-72 flex-col bg-surface-muted shadow-lg">
            <div className="flex items-center justify-end p-2">
              <Button
                variant="ghost"
                aria-label={t('shell.closeNav')}
                onClick={() => {
                  setMobileNavOpen(false);
                }}
              >
                <X className="size-5" />
              </Button>
            </div>
            <PlatformNavLinks
              pathname={pathname}
              onNavigate={() => {
                setMobileNavOpen(false);
              }}
            />
          </div>
        </div>
      ) : null}

      <main className="col-start-1 row-start-2 min-w-0 p-4 md:col-start-2 md:p-6">{children}</main>
      </div>
    </div>
  );
}
