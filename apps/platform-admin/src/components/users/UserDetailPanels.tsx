import { KeyRound, Shield, UserX } from 'lucide-react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import type { PlatformUser } from '@/lib/api/types';
import type { MessageKey } from '@/lib/i18n/messages';
import { cn } from '@/lib/utils';

type Translate = (key: MessageKey) => string;

type UserDetailActionsProps = {
  t: Translate;
  user: PlatformUser;
  busy: boolean;
  onDisable: () => void;
  onEnable: () => void;
  onResetPassword: () => void;
  onImpersonate: () => void;
};

export function UserProfileCard({ t, user }: { t: Translate; user: PlatformUser }) {
  return (
    <Card className="overflow-hidden p-0">
      <div className="border-b border-hairline bg-surface-muted/60 px-5 py-4">
        <p className="text-xs font-semibold uppercase tracking-[0.14em] text-muted-foreground">
          {t('users.profile')}
        </p>
        <div className="mt-3 flex flex-wrap items-center gap-2">
          <span
            className={cn(
              'rounded-full border px-2.5 py-0.5 text-xs font-semibold',
              user.active
                ? 'border-status-active/30 bg-status-active/10 text-status-active'
                : 'border-status-out-of-stock/30 bg-status-out-of-stock/10 text-status-out-of-stock',
            )}
          >
            {user.active ? t('users.active') : t('users.inactive')}
          </span>
          <span className="rounded-full border border-hairline bg-surface px-2.5 py-0.5 text-xs font-medium">
            {user.role}
          </span>
        </div>
      </div>
      <dl className="grid gap-4 px-5 py-5 sm:grid-cols-2">
        <DetailItem label={t('users.email')} value={user.email} />
        <DetailItem label={t('users.tenant')} value={user.tenant.displayName} />
        <DetailItem label={t('users.role')} value={user.role} />
        <DetailItem
          label={t('users.status')}
          value={user.active ? t('users.active') : t('users.inactive')}
        />
      </dl>
    </Card>
  );
}

export function UserDetailActions({
  t,
  user,
  busy,
  onDisable,
  onEnable,
  onResetPassword,
  onImpersonate,
}: UserDetailActionsProps) {
  return (
    <Card className="space-y-3 p-5">
      <p className="text-sm font-medium text-foreground">{t('users.actions')}</p>
      <p className="text-sm text-muted-foreground">{t('users.impersonateHint')}</p>
      <div className="flex flex-wrap gap-2">
        {user.active ? (
          <Button variant="secondary" onClick={onDisable} disabled={busy}>
            <UserX className="mr-1.5 size-4" aria-hidden />
            {t('users.disable')}
          </Button>
        ) : (
          <Button variant="secondary" onClick={onEnable} disabled={busy}>
            {t('users.enable')}
          </Button>
        )}
        <Button variant="secondary" onClick={onResetPassword} disabled={busy}>
          <KeyRound className="mr-1.5 size-4" aria-hidden />
          {t('users.resetPassword')}
        </Button>
        <Button onClick={onImpersonate} disabled={busy}>
          <Shield className="mr-1.5 size-4" aria-hidden />
          {t('users.impersonate')}
        </Button>
      </div>
    </Card>
  );
}

export function impersonationTargetUserId(user: PlatformUser): string | undefined {
  if (user.role === 'Admin' && user.active) {
    return user.id;
  }
  return undefined;
}

function DetailItem({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <dt className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
        {label}
      </dt>
      <dd className="mt-1 text-sm font-medium text-foreground">{value}</dd>
    </div>
  );
}
