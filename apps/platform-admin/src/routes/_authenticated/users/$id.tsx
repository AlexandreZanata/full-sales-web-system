import { createFileRoute } from '@tanstack/react-router';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { Textarea } from '@/components/ui/Textarea';
import { startImpersonation } from '@/lib/api/impersonate';
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

  const disable = useMutation({
    mutationFn: () => disablePlatformUser(id),
    onSuccess: () => void queryClient.invalidateQueries({ queryKey: ['platform-user', id] }),
  });
  const enable = useMutation({
    mutationFn: () => enablePlatformUser(id),
    onSuccess: () => void queryClient.invalidateQueries({ queryKey: ['platform-user', id] }),
  });
  const resetPassword = useMutation({
    mutationFn: () => resetPlatformUserPassword(id),
    onSuccess: (result) => {
      toast.success(`Temporary password: ${result.temporaryPassword}`);
    },
  });
  const impersonate = useMutation({
    mutationFn: () => {
      if (!user.data) {
        throw new Error('User not loaded');
      }
      return startImpersonation({ tenantId: user.data.tenantId, userId: id, reason });
    },
    onSuccess: () => {
      toast.success('Impersonation started');
      setImpersonateOpen(false);
    },
  });

  if (user.isLoading) {
    return <LoadingSpinner />;
  }
  if (!user.data) {
    return null;
  }

  return (
    <div className="space-y-4">
      <PageHeader
        title={user.data.name}
        back={<PageBackLink to="/users" label={t('users.title')} />}
      />
      <Card className="space-y-2 p-4 text-sm">
        <p>{user.data.email}</p>
        <p>{user.data.tenant.displayName}</p>
        <p>
          {user.data.role} · {user.data.active ? 'Active' : 'Inactive'}
        </p>
      </Card>
      <div className="flex flex-wrap gap-2">
        {user.data.active ? (
          <Button
            variant="secondary"
            onClick={() => {
              disable.mutate();
            }}
          >
            {t('users.disable')}
          </Button>
        ) : (
          <Button
            variant="secondary"
            onClick={() => {
              enable.mutate();
            }}
          >
            {t('users.enable')}
          </Button>
        )}
        <Button
          variant="secondary"
          onClick={() => {
            resetPassword.mutate();
          }}
        >
          {t('users.resetPassword')}
        </Button>
        <Button
          onClick={() => {
            setImpersonateOpen(true);
          }}
        >
          {t('users.impersonate')}
        </Button>
      </div>
      <ConfirmDialog
        open={impersonateOpen}
        title={t('users.impersonate')}
        message={t('users.impersonateReason')}
        onCancel={() => {
          setImpersonateOpen(false);
        }}
        onConfirm={() => {
          impersonate.mutate();
        }}
      />
      {impersonateOpen ? (
        <Textarea
          value={reason}
          onChange={(e) => {
            setReason(e.target.value);
          }}
        />
      ) : null}
    </div>
  );
}
