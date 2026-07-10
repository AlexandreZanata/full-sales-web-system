import { AlertTriangle } from 'lucide-react';

import { usePlatformAuth } from '@/auth/usePlatformAuth';
import { Button } from '@/components/ui/Button';
import { useI18n } from '@/lib/i18n/context';

export function ImpersonationBanner() {
  const { user } = usePlatformAuth();
  const { t } = useI18n();

  if (!user?.impersonating || !user.actingTenantId) {
    return null;
  }

  return (
    <div className="flex items-center gap-3 border-b border-status-warning/40 bg-status-warning/10 px-4 py-2 text-sm text-foreground">
      <AlertTriangle className="size-4 shrink-0 text-status-warning" aria-hidden />
      <span>
        {t('shell.impersonating')}: <strong>{user.actingTenantId}</strong>
      </span>
      <Button type="button" variant="secondary" className="ml-auto min-h-8 px-3 text-xs">
        {t('shell.endImpersonation')}
      </Button>
    </div>
  );
}
