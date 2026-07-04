import { useQuery } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { fetchCommercesForPicker } from '@/lib/api/commerces';
import type { User } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import { translateFormError, translateRole } from '@/lib/i18n/labels';
import { USER_ROLES, type UserRoleOption } from '@/lib/users/constants';
import {
  hasCreateUserFormErrors,
  toCreateUserPayload,
  validateCreateUserForm,
  type CreateUserFormValues,
} from '@/lib/users/validation';

type CreateUserFormProps = {
  onSubmit: (payload: ReturnType<typeof toCreateUserPayload>) => Promise<User>;
  onSuccess: (user: User) => void;
};

const emptyForm: CreateUserFormValues = {
  name: '',
  email: '',
  password: '',
  role: '',
  commerceId: '',
};

export function CreateUserForm({ onSubmit, onSuccess }: CreateUserFormProps) {
  const { t } = useI18n();
  const [values, setValues] = useState<CreateUserFormValues>(emptyForm);
  const [errors, setErrors] = useState<Partial<Record<keyof CreateUserFormValues, string>>>({});
  const [submitting, setSubmitting] = useState(false);
  const [submitError, setSubmitError] = useState<string | null>(null);

  const commerces = useQuery({
    queryKey: ['commerces', 'picker'],
    queryFn: fetchCommercesForPicker,
    enabled: values.role === 'CommerceContact',
  });

  function updateField<K extends keyof CreateUserFormValues>(
    key: K,
    value: CreateUserFormValues[K],
  ) {
    setValues((current) => ({ ...current, [key]: value }));
  }

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    setSubmitError(null);

    const nextErrors = validateCreateUserForm(values);
    setErrors(nextErrors);
    if (hasCreateUserFormErrors(nextErrors)) {
      return;
    }

    setSubmitting(true);
    try {
      const user = await onSubmit(toCreateUserPayload(values));
      onSuccess(user);
    } catch {
      setSubmitError(t('errors.actionFailed'));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <Card>
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <Input
          label={t('forms.fields.name')}
          name="name"
          value={values.name}
          error={translateFormError(t, errors.name)}
          onChange={(event) => {
            updateField('name', event.target.value);
          }}
        />
        <Input
          label={t('forms.fields.email')}
          name="email"
          type="email"
          autoComplete="off"
          value={values.email}
          error={translateFormError(t, errors.email)}
          onChange={(event) => {
            updateField('email', event.target.value);
          }}
        />
        <Input
          label={t('forms.fields.password')}
          name="password"
          type="password"
          autoComplete="new-password"
          value={values.password}
          error={translateFormError(t, errors.password)}
          onChange={(event) => {
            updateField('password', event.target.value);
          }}
        />
        <Select
          label={t('forms.fields.role')}
          name="role"
          value={values.role}
          error={translateFormError(t, errors.role)}
          onChange={(event) => {
            const role = event.target.value as UserRoleOption | '';
            updateField('role', role);
            if (role !== 'CommerceContact') {
              updateField('commerceId', '');
            }
          }}
        >
          <option value="">{t('forms.placeholders.selectRole')}</option>
          {USER_ROLES.map((role) => (
            <option key={role} value={role}>
              {translateRole(t, role)}
            </option>
          ))}
        </Select>

        {values.role === 'CommerceContact' ? (
          <Select
            label={t('forms.fields.commerce')}
            name="commerceId"
            value={values.commerceId}
            error={translateFormError(t, errors.commerceId)}
            disabled={commerces.isLoading}
            onChange={(event) => {
              updateField('commerceId', event.target.value);
            }}
          >
            <option value="">{t('forms.placeholders.selectCommerce')}</option>
            {commerces.data?.map((commerce) => (
              <option key={commerce.id} value={commerce.id}>
                {commerce.tradeName || commerce.legalName}
              </option>
            ))}
          </Select>
        ) : null}

        {submitError ? <p className="text-sm text-destructive">{submitError}</p> : null}

        <Button type="submit" disabled={submitting}>
          {submitting ? t('users.create.submitting') : t('users.create.submit')}
        </Button>
      </form>
    </Card>
  );
}
