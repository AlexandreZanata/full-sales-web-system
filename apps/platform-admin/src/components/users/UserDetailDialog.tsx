import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import {
  UserDetailActions,
  UserProfileCard,
  impersonationTargetUserId,
} from '@/components/users/UserDetailPanels';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { Dialog } from '@/components/ui/Dialog';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { Textarea } from '@/components/ui/Textarea';
import { openTenantAdminWithImpersonation, startImpersonation } from '@/lib/api/impersonate';
import {
  disablePlatformUser,
  enablePlatformUser,
  fetchPlatformUser,
  resetPlatformUserPassword,
} from '@/lib/api/users';
import { useI18n } from '@/lib/i18n/context';
import { useToast } from '@/hooks/useToast';

type UserDetailDialogProps = {
  userId: string | null;
  onClose: () => void;
};

function requireUserId(userId: string | null): string {
  if (!userId) {
    throw new Error('userId required');
  }
  return userId;
}

export function UserDetailDialog({ userId, onClose }: UserDetailDialogProps) {
  const { t } = useI18n();
  const toast = useToast();
  const queryClient = useQueryClient();
  const [impersonateOpen, setImpersonateOpen] = useState(false);
  const [reason, setReason] = useState('');
  const open = Boolean(userId);

  const user = useQuery({
    queryKey: ['platform-user', userId],
    queryFn: () => fetchPlatformUser(requireUserId(userId)),
    enabled: open && Boolean(userId),
  });

  const invalidate = () => {
    void queryClient.invalidateQueries({ queryKey: ['platform-user', userId] });
    void queryClient.invalidateQueries({ queryKey: ['platform-users'] });
  };

  const disable = useMutation({
    mutationFn: () => disablePlatformUser(requireUserId(userId)),
    onSuccess: invalidate,
    onError: () => {
      toast.error(t('common.somethingWentWrong'));
    },
  });
  const enable = useMutation({
    mutationFn: () => enablePlatformUser(requireUserId(userId)),
    onSuccess: invalidate,
    onError: () => {
      toast.error(t('common.somethingWentWrong'));
    },
  });
  const resetPassword = useMutation({
    mutationFn: () => resetPlatformUserPassword(requireUserId(userId)),
    onSuccess: (result) => {
      toast.success(`Temporary password: ${result.temporaryPassword}`);
    },
    onError: () => {
      toast.error(t('common.somethingWentWrong'));
    },
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
    onError: () => {
      toast.error(t('users.impersonateFailed'));
    },
  });

  const busy =
    disable.isPending || enable.isPending || resetPassword.isPending || impersonate.isPending;

  return (
    <>
      <Dialog
        open={open}
        title={user.data?.name ?? t('users.profile')}
        onClose={onClose}
        className="max-w-2xl"
      >
        {user.isLoading || !user.data ? (
          <LoadingSpinner />
        ) : (
          <div className="space-y-4">
            <p className="text-sm text-muted-foreground">{user.data.email}</p>
            <UserProfileCard t={t} user={user.data} />
            <UserDetailActions
              t={t}
              user={user.data}
              busy={busy}
              onDisable={() => {
                disable.mutate();
              }}
              onEnable={() => {
                enable.mutate();
              }}
              onResetPassword={() => {
                resetPassword.mutate();
              }}
              onImpersonate={() => {
                setImpersonateOpen(true);
              }}
            />
          </div>
        )}
      </Dialog>
      <ConfirmDialog
        open={impersonateOpen}
        title={t('users.impersonate')}
        message={t('users.impersonateDialogBody')}
        confirmDisabled={reason.trim().length < 3}
        isLoading={impersonate.isPending}
        onCancel={() => {
          setImpersonateOpen(false);
          setReason('');
        }}
        onConfirm={() => {
          impersonate.mutate();
        }}
      >
        <Textarea
          label={t('users.impersonateReason')}
          value={reason}
          placeholder={t('users.impersonateReasonPlaceholder')}
          onChange={(e) => {
            setReason(e.target.value);
          }}
        />
      </ConfirmDialog>
    </>
  );
}
