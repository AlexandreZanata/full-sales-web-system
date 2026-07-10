import { useRouterState } from '@tanstack/react-router';
import { LogOut, Menu, X } from 'lucide-react';
import { useEffect, useState, type ReactNode } from 'react';

import { useAdminAuth } from '@/auth/useAdminAuth';
import { AdminBrand, AdminNavLinks } from '@/components/AdminNavLinks';
import { BillingStatusBanner } from '@/components/settings/BillingStatusBanner';
import { SiteBrand } from '@/components/SiteBrand';
import { LocaleSwitcher } from '@/components/LocaleSwitcher';
import { Button } from '@/components/ui/Button';
import { useI18n } from '@/lib/i18n/context';
import { adminTokens } from '@/lib/admin-tokens';
import { cn } from '@/lib/utils';

type AdminShellProps = {
  children: ReactNode;
};

export function AdminShell({ children }: AdminShellProps) {
  const { user, logout } = useAdminAuth();
  const { t } = useI18n();
  const [mobileNavOpen, setMobileNavOpen] = useState(false);
  const pathname = useRouterState({ select: (state) => state.location.pathname });

  useEffect(() => {
    setMobileNavOpen(false);
  }, [pathname]);

  useEffect(() => {
    if (!mobileNavOpen) {
      return;
    }
    const previousOverflow = document.body.style.overflow;
    document.body.style.overflow = 'hidden';
    return () => {
      document.body.style.overflow = previousOverflow;
    };
  }, [mobileNavOpen]);

  async function handleLogout() {
    await logout();
    window.location.assign('/login');
  }

  return (
    <div className="flex min-h-dvh flex-col bg-background md:grid md:grid-cols-[15rem_1fr] md:grid-rows-[6.25rem_minmax(0,1fr)]">
      <header className={cn(adminTokens.shellHeaderBar, 'md:hidden')}>
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
        <div className="flex min-w-0 flex-1 items-center">
          <SiteBrand subtitleKey="auth.adminLabel" className="border-0 px-0 py-0" />
        </div>
        <Button
          variant="ghost"
          className="min-h-10 min-w-10"
          aria-label={t('auth.logout')}
          onClick={() => {
            void handleLogout();
          }}
        >
          <LogOut className="size-4" />
        </Button>
      </header>

      <AdminBrand className="col-start-1 row-start-1 hidden md:flex" />
      <header className={cn('col-start-2 row-start-1 hidden md:flex', adminTokens.shellHeaderBar)}>
        <LocaleSwitcher className="mr-auto" />
        <span className="text-sm text-muted-foreground">{user?.email}</span>
        <Button
          variant="ghost"
          onClick={() => {
            void handleLogout();
          }}
        >
          <LogOut className="size-4" />
          {t('auth.logout')}
        </Button>
      </header>
      <aside className="col-start-1 row-start-2 hidden flex-col border-r border-hairline bg-surface md:flex">
        <AdminNavLinks className="flex-1 p-3" />
      </aside>
      <main className="flex-1 px-4 py-5 pb-[max(1.25rem,env(safe-area-inset-bottom))] md:col-start-2 md:row-start-2 md:px-6 md:py-6">
        <BillingStatusBanner />
        {children}
      </main>

      {mobileNavOpen ? (
        <div
          className="fixed inset-0 z-50 md:hidden"
          role="dialog"
          aria-modal="true"
          aria-label={t('shell.navMenu')}
        >
          <button
            type="button"
            className="absolute inset-0 bg-foreground/40"
            aria-label={t('shell.closeNav')}
            onClick={() => {
              setMobileNavOpen(false);
            }}
          />
          <aside className="relative flex h-full w-[min(100%,18rem)] flex-col bg-surface shadow-lg">
            <div className="flex items-center justify-between border-b border-hairline px-4 py-3">
              <span className="text-sm font-semibold text-foreground">{t('shell.menu')}</span>
              <Button
                type="button"
                variant="ghost"
                className="min-h-10 min-w-10"
                aria-label={t('shell.closeMenu')}
                onClick={() => {
                  setMobileNavOpen(false);
                }}
              >
                <X className="size-5" />
              </Button>
            </div>
            <AdminBrand />
            <AdminNavLinks
              className="flex-1 p-3"
              onNavigate={() => {
                setMobileNavOpen(false);
              }}
            />
          </aside>
        </div>
      ) : null}
    </div>
  );
}
