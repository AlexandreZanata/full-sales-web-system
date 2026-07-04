import { Link } from '@tanstack/react-router';

import { BrandMark } from '@/components/BrandMark';
import { adminTokens } from '@/lib/admin-tokens';
import { BRAND_NAME } from '@/lib/brand';
import { adminNavItems } from '@/lib/admin-nav';
import { cn } from '@/lib/utils';

type AdminNavLinksProps = {
  onNavigate?: () => void;
  className?: string;
};

export function AdminNavLinks({ onNavigate, className }: AdminNavLinksProps) {
  return (
    <nav
      data-admin-sidebar
      aria-label="Admin navigation"
      className={cn('flex flex-col gap-1', className)}
    >
      {adminNavItems.map(({ to, label, icon: Icon }) => (
        <Link
          key={to}
          to={to}
          activeOptions={to === '/' ? { exact: true } : undefined}
          className={adminTokens.sidebarItem}
          activeProps={{ className: adminTokens.sidebarActive }}
          onClick={onNavigate}
        >
          <Icon className="size-4 shrink-0" aria-hidden />
          {label}
        </Link>
      ))}
    </nav>
  );
}

export function AdminBrand({ className }: { className?: string }) {
  return (
    <div className={cn(adminTokens.shellBrandBar, className)}>
      <div className="flex items-center gap-3">
        <BrandMark size="md" />
        <div className="min-w-0">
          <p className="text-xs font-semibold uppercase tracking-[0.2em] text-muted-foreground">
            {BRAND_NAME}
          </p>
          <p className="mt-1 text-lg font-semibold leading-tight text-foreground">Admin</p>
        </div>
      </div>
    </div>
  );
}
