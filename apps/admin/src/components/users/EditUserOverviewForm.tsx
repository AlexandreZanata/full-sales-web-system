import { useState, type ReactNode, type SubmitEvent } from 'react';

import { ActiveBadge } from '@/components/users/ActiveBadge';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { ApiError } from '@/lib/api/client';
import type { User } from '@/lib/api/types';
import { updateUser } from '@/lib/api/users';
import { useI18n } from '@/lib/i18n/context';
import { translateRole } from '@/lib/i18n/labels';

type EditUserOverviewFormProps = {
  user: User;
  onSaved: (user: User) => void;
};

export function EditUserOverviewForm({ user, onSaved }: EditUserOverviewFormProps) {
  const { t } = useI18n();
  const [editing, setEditing] = useState(false);
  const [name, setName] = useState(user.name);
  const [email, setEmail] = useState(user.email);
  const [password, setPassword] = useState('');
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  function startEdit() {
    setName(user.name);
    setEmail(user.email);
    setPassword('');
    setError(null);
    setEditing(true);
  }

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    setError(null);
    setSubmitting(true);
    try {
      const updated = await updateUser(user.id, {
        name: name.trim(),
        email: email.trim(),
        password: password.trim() || undefined,
      });
      onSaved(updated);
      setEditing(false);
      setPassword('');
    } catch (err) {
      setError(err instanceof ApiError ? err.message : t('errors.actionFailed'));
    } finally {
      setSubmitting(false);
    }
  }

  if (!editing) {
    return (
      <Card className="space-y-3">
        <div className="flex justify-end">
          <Button type="button" variant="secondary" onClick={startEdit}>
            {t('users.detail.edit')}
          </Button>
        </div>
        <DetailRow label={t('forms.fields.name')} value={user.name} />
        <DetailRow label={t('forms.fields.email')} value={user.email} />
        <DetailRow label={t('forms.fields.role')} value={translateRole(t, user.role)} />
        <DetailRow label={t('forms.fields.status')} value={<ActiveBadge active={user.active} />} />
        {user.commerceId ? (
          <DetailRow label={t('forms.fields.commerceId')} value={user.commerceId} />
        ) : null}
      </Card>
    );
  }

  return (
    <Card>
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <Input
          label={t('forms.fields.name')}
          name="name"
          required
          value={name}
          onChange={(event) => {
            setName(event.target.value);
          }}
        />
        <Input
          label={t('forms.fields.email')}
          name="email"
          type="email"
          required
          value={email}
          onChange={(event) => {
            setEmail(event.target.value);
          }}
        />
        <Input
          label={t('users.detail.newPassword')}
          name="password"
          type="password"
          autoComplete="new-password"
          value={password}
          onChange={(event) => {
            setPassword(event.target.value);
          }}
        />
        <p className="-mt-2 text-xs text-muted-foreground">{t('users.detail.newPasswordHint')}</p>
        <DetailRow label={t('forms.fields.role')} value={translateRole(t, user.role)} />
        <p className="text-xs text-muted-foreground">{t('users.detail.roleReadonly')}</p>
        <DetailRow label={t('forms.fields.status')} value={<ActiveBadge active={user.active} />} />
        {error ? <p className="text-sm text-destructive">{error}</p> : null}
        <div className="flex flex-wrap gap-2">
          <Button type="submit" disabled={submitting}>
            {submitting ? t('users.detail.saving') : t('users.detail.save')}
          </Button>
          <Button
            type="button"
            variant="secondary"
            disabled={submitting}
            onClick={() => {
              setEditing(false);
            }}
          >
            {t('common.cancel')}
          </Button>
        </div>
      </form>
    </Card>
  );
}

function DetailRow({ label, value }: { label: string; value: ReactNode }) {
  return (
    <div className="flex flex-col gap-1 sm:flex-row sm:items-center sm:justify-between">
      <span className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
        {label}
      </span>
      <span className="text-sm text-foreground">{value}</span>
    </div>
  );
}
