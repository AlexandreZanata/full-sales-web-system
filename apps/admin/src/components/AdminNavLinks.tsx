import { Link } from '@tanstack/react-router';

import { SiteBrand } from '@/components/SiteBrand';
import { useI18n } from '@/lib/i18n/context';
import { adminTokens } from '@/lib/admin-tokens';
import { adminNavItems } from '@/lib/admin-nav';
import { cn } from '@/lib/utils';

type AdminNavLinksProps = {
  onNavigate?: () => void;
  className?: string;
};

export function AdminNavLinks({ onNavigate, className }: AdminNavLinksProps) {
  const { t } = useI18n();

  return (
    <nav
      data-admin-sidebar
      aria-label={t('shell.adminNav')}
      className={cn('flex flex-col gap-1', className)}
    >
      {adminNavItems.map(({ to, labelKey, icon: Icon }) => (
        <Link
          key={to}
          to={to}
          activeOptions={to === '/' ? { exact: true } : undefined}
          className={adminTokens.sidebarItem}
          activeProps={{ className: adminTokens.sidebarActive }}
          onClick={onNavigate}
        >
          <Icon className="size-4 shrink-0" aria-hidden />
          {t(labelKey)}
        </Link>
      ))}
    </nav>
  );
}

export function AdminBrand({ className }: { className?: string }) {
  return <SiteBrand className={className} subtitleKey="auth.adminLabel" />;
}
