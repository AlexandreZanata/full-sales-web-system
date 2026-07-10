import { Link } from '@tanstack/react-router';
import { type LucideIcon } from 'lucide-react';

import { BrandMark } from '@/components/BrandMark';
import { useI18n } from '@/lib/i18n/context';
import { PLATFORM_APP_TITLE } from '@/lib/brand';
import { platformNavItems } from '@/lib/platform-nav';
import { platformTokens } from '@/lib/platform-tokens';
import { cn } from '@/lib/utils';

type PlatformBrandProps = {
  className?: string;
};

export function PlatformBrand({ className }: PlatformBrandProps) {
  const { t } = useI18n();
  return (
    <div className={cn(platformTokens.shellSidebar, 'px-4 py-5', className)}>
      <div className="flex items-center gap-3">
        <BrandMark />
        <div className="min-w-0">
          <p className="truncate text-sm font-semibold text-foreground">{PLATFORM_APP_TITLE}</p>
          <p className="truncate text-xs text-muted-foreground">{t('auth.platformLabel')}</p>
        </div>
      </div>
    </div>
  );
}

type PlatformNavLinksProps = {
  pathname: string;
  onNavigate?: () => void;
};

export function PlatformNavLinks({ pathname, onNavigate }: PlatformNavLinksProps) {
  const { t } = useI18n();

  return (
    <nav aria-label={t('shell.navMenu')} className="flex flex-1 flex-col gap-1 px-3 py-4">
      {platformNavItems.map((item) => {
        const active = item.to === '/' ? pathname === '/' : pathname.startsWith(item.to);
        return (
          <NavLink
            key={item.to}
            to={item.to}
            icon={item.icon}
            active={active}
            onNavigate={onNavigate}
          >
            {t(item.labelKey)}
          </NavLink>
        );
      })}
    </nav>
  );
}

function NavLink({
  to,
  icon: Icon,
  active,
  children,
  onNavigate,
}: {
  to: string;
  icon: LucideIcon;
  active: boolean;
  children: string;
  onNavigate?: () => void;
}) {
  return (
    <Link
      to={to}
      className={cn(platformTokens.shellNavLink, active && platformTokens.shellNavLinkActive)}
      onClick={onNavigate}
    >
      <Icon className="size-4 shrink-0" aria-hidden />
      <span>{children}</span>
    </Link>
  );
}
