import { Link, useRouterState } from '@tanstack/react-router';

import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

const SETTINGS_LINKS = [
  { to: '/settings', labelKey: 'settings.nav.general' as const, exact: true },
  { to: '/settings/billing', labelKey: 'settings.nav.billing' as const, exact: false },
  { to: '/settings/payments', labelKey: 'settings.nav.payments' as const, exact: false },
  { to: '/settings/domains', labelKey: 'settings.nav.domains' as const, exact: false },
];

export function SettingsNav() {
  const { t } = useI18n();
  const pathname = useRouterState({ select: (s) => s.location.pathname });

  return (
    <nav
      aria-label={t('settings.nav.aria')}
      className="flex flex-wrap gap-2 border-b border-hairline pb-4"
    >
      {SETTINGS_LINKS.map((item) => {
        const active = item.exact
          ? pathname === '/settings' || pathname === '/settings/'
          : pathname.startsWith(item.to);
        return (
          <Link
            key={item.to}
            to={item.to}
            className={cn(
              'rounded-md px-3 py-2 text-sm font-medium transition',
              active
                ? 'bg-surface text-foreground shadow-sm'
                : 'text-muted-foreground hover:text-foreground',
            )}
          >
            {t(item.labelKey)}
          </Link>
        );
      })}
    </nav>
  );
}
