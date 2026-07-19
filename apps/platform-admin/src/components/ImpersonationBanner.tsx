import { AlertTriangle } from 'lucide-react';
import { useSyncExternalStore } from 'react';

import { Button } from '@/components/ui/Button';
import { endImpersonation } from '@/lib/api/impersonate';
import {
  clearStoredImpersonation,
  getStoredImpersonation,
  subscribeImpersonation,
  type StoredImpersonation,
} from '@/lib/auth/sessionUser';
import { useI18n } from '@/lib/i18n/context';

function getSnapshot(): StoredImpersonation | null {
  return getStoredImpersonation();
}

export function ImpersonationBanner() {
  const { t } = useI18n();
  const stored = useSyncExternalStore(subscribeImpersonation, getSnapshot, () => null);

  if (!stored || !stored.tenantId) {
    return null;
  }

  const { tenantId, grantId } = stored;

  async function handleEnd() {
    try {
      if (grantId) {
        await endImpersonation(grantId);
      } else {
        clearStoredImpersonation();
      }
    } catch {
      clearStoredImpersonation();
    }
  }

  return (
    <div className="flex items-center gap-3 border-b border-status-warning/40 bg-status-warning/10 px-4 py-2 text-sm text-foreground">
      <AlertTriangle className="size-4 shrink-0 text-status-warning" aria-hidden />
      <span>
        {t('shell.impersonating')}: <strong>{tenantId}</strong>
      </span>
      <Button
        type="button"
        variant="secondary"
        className="ml-auto min-h-8 px-3 text-xs"
        onClick={() => void handleEnd()}
      >
        {t('shell.endImpersonation')}
      </Button>
    </div>
  );
}
