import { type ReactNode } from 'react';

import { PortalFooter } from '@/components/layout/PortalFooter';
import { PortalHeader } from '@/components/layout/PortalHeader';
import { PortalMobileTabBar } from '@/components/layout/PortalMobileTabBar';

type PortalShellProps = {
  children: ReactNode;
};

export function PortalShell({ children }: PortalShellProps) {
  return (
    <div className="portal-app-shell">
      <PortalHeader />
      <main className="portal-app-main">{children}</main>
      <div className="hidden lg:block">
        <PortalFooter />
      </div>
      <PortalMobileTabBar />
    </div>
  );
}
