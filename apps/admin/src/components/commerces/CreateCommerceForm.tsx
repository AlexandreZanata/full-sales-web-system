import { useState, type SubmitEvent } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import type { Commerce, CreateCommerceRequest } from '@/lib/api/types';
import {
  formatCnpjInput,
  hasFormErrors,
  toCreateCommercePayload,
  validateCreateCommerceForm,
  type CreateCommerceFormValues,
} from '@/lib/commerces/validation';
import { useI18n } from '@/lib/i18n/context';
import { translateFormError } from '@/lib/i18n/labels';

type CreateCommerceFormProps = {
  onSubmit: (payload: CreateCommerceRequest) => Promise<Commerce>;
  onSuccess: (commerce: Commerce) => void;
};

const emptyForm: CreateCommerceFormValues = {
  cnpj: '',
  legalName: '',
  tradeName: '',
  street: '',
  number: '',
  district: '',
  city: '',
  state: '',
  postalCode: '',
  contactPhone: '',
  contactEmail: '',
};

export function CreateCommerceForm({ onSubmit, onSuccess }: CreateCommerceFormProps) {
  const { t } = useI18n();
  const toast = useToast();
  const [values, setValues] = useState<CreateCommerceFormValues>(emptyForm);
  const [errors, setErrors] = useState<Partial<Record<keyof CreateCommerceFormValues, string>>>({});
  const [submitting, setSubmitting] = useState(false);

  function updateField<K extends keyof CreateCommerceFormValues>(
    key: K,
    value: CreateCommerceFormValues[K],
  ) {
    setValues((current) => ({ ...current, [key]: value }));
  }

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const nextErrors = validateCreateCommerceForm(values);
    setErrors(nextErrors);
    if (hasFormErrors(nextErrors)) {
      return;
    }

    setSubmitting(true);
    try {
      const commerce = await onSubmit(toCreateCommercePayload(values));
      onSuccess(commerce);
    } catch (error) {
      if (error instanceof ApiError && error.code === 'INVALID_CNPJ') {
        const message = t('forms.validation.cnpjInvalid');
        setErrors((current) => ({ ...current, cnpj: message }));
        toast.error(message);
        return;
      }
      toast.error(t('errors.actionFailed'));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <Card>
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <div className="grid gap-4 sm:grid-cols-2">
          <Input
            label={t('forms.fields.cnpj')}
            name="cnpj"
            inputMode="numeric"
            autoComplete="off"
            value={values.cnpj}
            error={translateFormError(t, errors.cnpj)}
            onChange={(event) => {
              updateField('cnpj', formatCnpjInput(event.target.value));
            }}
          />
          <Input
            label={t('forms.fields.legalName')}
            name="legalName"
            value={values.legalName}
            error={translateFormError(t, errors.legalName)}
            onChange={(event) => {
              updateField('legalName', event.target.value);
            }}
          />
          <Input
            label={t('forms.fields.tradeName')}
            name="tradeName"
            value={values.tradeName}
            onChange={(event) => {
              updateField('tradeName', event.target.value);
            }}
          />
        </div>

        <fieldset className="space-y-4 rounded-lg border border-hairline p-4">
          <legend className="px-1 text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
            {t('forms.sections.address')}
          </legend>
          <div className="grid gap-4 sm:grid-cols-2">
            <Input
              label={t('forms.fields.street')}
              name="street"
              value={values.street}
              error={translateFormError(t, errors.street)}
              onChange={(event) => {
                updateField('street', event.target.value);
              }}
            />
            <Input
              label={t('forms.fields.number')}
              name="number"
              value={values.number}
              error={translateFormError(t, errors.number)}
              onChange={(event) => {
                updateField('number', event.target.value);
              }}
            />
            <Input
              label={t('forms.fields.district')}
              name="district"
              value={values.district}
              onChange={(event) => {
                updateField('district', event.target.value);
              }}
            />
            <Input
              label={t('forms.fields.city')}
              name="city"
              value={values.city}
              error={translateFormError(t, errors.city)}
              onChange={(event) => {
                updateField('city', event.target.value);
              }}
            />
            <Input
              label={t('forms.fields.state')}
              name="state"
              maxLength={2}
              value={values.state}
              error={translateFormError(t, errors.state)}
              onChange={(event) => {
                updateField('state', event.target.value.toUpperCase());
              }}
            />
            <Input
              label={t('forms.fields.postalCode')}
              name="postalCode"
              inputMode="numeric"
              value={values.postalCode}
              error={translateFormError(t, errors.postalCode)}
              onChange={(event) => {
                updateField('postalCode', event.target.value.replace(/\D/g, '').slice(0, 8));
              }}
            />
          </div>
        </fieldset>

        <fieldset className="space-y-4 rounded-lg border border-hairline p-4">
          <legend className="px-1 text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
            {t('forms.sections.contact')}
          </legend>
          <div className="grid gap-4 sm:grid-cols-2">
            <Input
              label={t('forms.fields.phone')}
              name="contactPhone"
              type="tel"
              value={values.contactPhone}
              onChange={(event) => {
                updateField('contactPhone', event.target.value);
              }}
            />
            <Input
              label={t('forms.fields.email')}
              name="contactEmail"
              type="email"
              value={values.contactEmail}
              error={translateFormError(t, errors.contactEmail)}
              onChange={(event) => {
                updateField('contactEmail', event.target.value);
              }}
            />
          </div>
        </fieldset>

        <Button type="submit" disabled={submitting}>
          {submitting ? t('commerces.create.submitting') : t('commerces.create.submit')}
        </Button>
      </form>
    </Card>
  );
}
