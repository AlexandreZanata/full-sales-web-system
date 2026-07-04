import { useQuery } from '@tanstack/react-query';
import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { fetchCommercesForPicker } from '@/lib/api/commerces';
import type { User } from '@/lib/api/types';
import { USER_ROLE_LABELS, USER_ROLES, type UserRoleOption } from '@/lib/users/constants';
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
      setSubmitError('Unable to create user. Check the form and try again.');
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <Card>
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <Input
          label="Name"
          name="name"
          value={values.name}
          error={errors.name}
          onChange={(event) => {
            updateField('name', event.target.value);
          }}
        />
        <Input
          label="Email"
          name="email"
          type="email"
          autoComplete="off"
          value={values.email}
          error={errors.email}
          onChange={(event) => {
            updateField('email', event.target.value);
          }}
        />
        <Input
          label="Password"
          name="password"
          type="password"
          autoComplete="new-password"
          value={values.password}
          error={errors.password}
          onChange={(event) => {
            updateField('password', event.target.value);
          }}
        />
        <Select
          label="Role"
          name="role"
          value={values.role}
          error={errors.role}
          onChange={(event) => {
            const role = event.target.value as UserRoleOption | '';
            updateField('role', role);
            if (role !== 'CommerceContact') {
              updateField('commerceId', '');
            }
          }}
        >
          <option value="">Select role</option>
          {USER_ROLES.map((role) => (
            <option key={role} value={role}>
              {USER_ROLE_LABELS[role]}
            </option>
          ))}
        </Select>

        {values.role === 'CommerceContact' ? (
          <Select
            label="Commerce"
            name="commerceId"
            value={values.commerceId}
            error={errors.commerceId}
            disabled={commerces.isLoading}
            onChange={(event) => {
              updateField('commerceId', event.target.value);
            }}
          >
            <option value="">Select commerce</option>
            {commerces.data?.map((commerce) => (
              <option key={commerce.id} value={commerce.id}>
                {commerce.tradeName || commerce.legalName}
              </option>
            ))}
          </Select>
        ) : null}

        {submitError ? <p className="text-sm text-destructive">{submitError}</p> : null}

        <Button type="submit" disabled={submitting}>
          {submitting ? 'Creating…' : 'Create user'}
        </Button>
      </form>
    </Card>
  );
}
