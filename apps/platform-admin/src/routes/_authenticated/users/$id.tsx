import { createFileRoute } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import {
  UserDetailActions,
  UserProfileCard,
  impersonationTargetUserId,
} from '@/components/users/UserDetailPanels';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { Textarea } from '@/components/ui/Textarea';
import {
  openTenantAdminWithImpersonation,
  startImpersonation,
} from '@/lib/api/impersonate';
import {
  disablePlatformUser,
  enablePlatformUser,
  fetchPlatformUser,
  resetPlatformUserPassword,
} from '@/lib/api/users';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

export const Route = createFileRoute('/_authenticated/users/$id')({
  component: UserDetailPage,
});

function UserDetailPage() {
  const { id } = Route.useParams();
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [impersonateOpen, setImpersonateOpen] = useState(false);
  const [reason, setReason] = useState('');

  const user = useQuery({ queryKey: ['platform-user', id], queryFn: () => fetchPlatformUser(id) });
  const invalidate = () => void queryClient.invalidateQueries({ queryKey: ['platform-user', id] });

  const disable = useMutation({
    mutationFn: () => disablePlatformUser(id),
    onSuccess: invalidate,
    onError: () => toast.error(t('common.somethingWentWrong')),
  });
  const enable = useMutation({
    mutationFn: () => enablePlatformUser(id),
    onSuccess: invalidate,
    onError: () => toast.error(t('common.somethingWentWrong')),
  });
  const resetPassword = useMutation({
    mutationFn: () => resetPlatformUserPassword(id),
    onSuccess: (result) => toast.success(`Temporary password: ${result.temporaryPassword}`),
    onError: () => toast.error(t('common.somethingWentWrong')),
  });
  const impersonate = useMutation({
    mutationFn: () => {
      if (!user.data) {
        throw new Error('User not loaded');
      }
      return startImpersonation({
        tenantId: user.data.tenantId,
        userId: impersonationTargetUserId(user.data),
        reason: reason.trim(),
      });
    },
    onSuccess: (result) => {
      openTenantAdminWithImpersonation(result.impersonationToken);
      setImpersonateOpen(false);
      setReason('');
      toast.success(t('users.impersonateOpened'));
    },
    onError: () => toast.error(t('users.impersonateFailed')),
  });

  if (user.isLoading) {
    return (
      <div className="flex justify-center py-16">
        <LoadingSpinner />
      </div>
    );
  }
  if (!user.data) {
    return null;
  }

  const busy =
    disable.isPending || enable.isPending || resetPassword.isPending || impersonate.isPending;

  return (
    <div className="mx-auto max-w-3xl space-y-6">
      <PageHeader
        title={user.data.name}
        description={user.data.email}
        back={<PageBackLink to="/users" label={t('users.title')} />}
      />
      <UserProfileCard t={t} user={user.data} />
      <UserDetailActions
        t={t}
        user={user.data}
        busy={busy}
        onDisable={() => disable.mutate()}
        onEnable={() => enable.mutate()}
        onResetPassword={() => resetPassword.mutate()}
        onImpersonate={() => setImpersonateOpen(true)}
      />
      <ConfirmDialog
        open={impersonateOpen}
        title={t('users.impersonate')}
        message={t('users.impersonateDialogBody')}
        confirmLabel={t('users.impersonate')}
        isLoading={impersonate.isPending}
        confirmDisabled={reason.trim().length < 3}
        onCancel={() => {
          setImpersonateOpen(false);
          setReason('');
        }}
        onConfirm={() => impersonate.mutate()}
      >
        <Textarea
          label={t('users.impersonateReason')}
          value={reason}
          placeholder={t('users.impersonateReasonPlaceholder')}
          onChange={(event) => setReason(event.target.value)}
        />
      </ConfirmDialog>
    </div>
  );
}
