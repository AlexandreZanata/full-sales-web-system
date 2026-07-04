import { useState, type SubmitEvent } from 'react';

import { AddressFormFields } from '@/components/commerces/AddressFormFields';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { useToast } from '@/hooks/useToast';
import { createCommerceAddress, updateCommerceAddress } from '@/lib/api/commerces';
import type { CommerceAddress } from '@/lib/api/types';
import {
  hasFormErrors,
  toAddressPayload,
  toUpdateAddressPayload,
  validateAddressForm,
  type AddressFormValues,
} from '@/lib/commerces/validation';
import { useI18n } from '@/lib/i18n/context';
import { translateAddressType } from '@/lib/i18n/labels';

type AddressEditorProps = {
  commerceId: string;
  initial?: CommerceAddress;
  onSaved: () => void;
  onCancel?: () => void;
};

const emptyAddressForm: AddressFormValues = {
  addressType: '',
  street: '',
  number: '',
  district: '',
  city: '',
  state: '',
  postalCode: '',
  isPrimary: false,
};

function toFormValues(address: CommerceAddress): AddressFormValues {
  return {
    addressType: address.addressType,
    street: address.street,
    number: address.number,
    district: address.district ?? '',
    city: address.city,
    state: address.state,
    postalCode: address.postalCode,
    isPrimary: address.isPrimary,
  };
}

function AddressEditor({ commerceId, initial, onSaved, onCancel }: AddressEditorProps) {
  const { t } = useI18n();
  const toast = useToast();
  const [values, setValues] = useState<AddressFormValues>(
    initial ? toFormValues(initial) : emptyAddressForm,
  );
  const [errors, setErrors] = useState<Partial<Record<keyof AddressFormValues, string>>>({});
  const [submitting, setSubmitting] = useState(false);

  function updateField<K extends keyof AddressFormValues>(key: K, value: AddressFormValues[K]) {
    setValues((current) => ({ ...current, [key]: value }));
  }

  async function handleSubmit(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    const nextErrors = validateAddressForm(values);
    setErrors(nextErrors);
    if (hasFormErrors(nextErrors)) {
      return;
    }

    setSubmitting(true);
    try {
      if (initial) {
        await updateCommerceAddress(commerceId, initial.id, toUpdateAddressPayload(values));
        toast.success(t('commerces.toast.addressUpdated'));
      } else {
        await createCommerceAddress(commerceId, toAddressPayload(values));
        toast.success(t('commerces.toast.addressAdded'));
      }
      onSaved();
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <Card className="space-y-4">
      <form className="space-y-4" onSubmit={(event) => void handleSubmit(event)}>
        <AddressFormFields
          values={values}
          errors={errors}
          showTypeField={!initial}
          onChange={updateField}
        />
        <div className="flex flex-wrap gap-2">
          <Button type="submit" disabled={submitting}>
            {submitting
              ? t('commerces.addresses.saving')
              : initial
                ? t('commerces.addresses.save')
                : t('commerces.addresses.add')}
          </Button>
          {onCancel ? (
            <Button type="button" variant="secondary" onClick={onCancel}>
              {t('common.cancel')}
            </Button>
          ) : null}
        </div>
      </form>
    </Card>
  );
}

type AddressesTabProps = {
  commerceId: string;
  addresses: CommerceAddress[];
  onChanged: () => void;
};

export function AddressesTab({ commerceId, addresses, onChanged }: AddressesTabProps) {
  const { t } = useI18n();
  const [showAddForm, setShowAddForm] = useState(false);
  const [editingId, setEditingId] = useState<string | null>(null);

  return (
    <div className="space-y-4">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <p className="text-sm text-muted-foreground">{t('commerces.addresses.description')}</p>
        {!showAddForm ? (
          <Button
            type="button"
            onClick={() => {
              setShowAddForm(true);
              setEditingId(null);
            }}
          >
            {t('commerces.addresses.add')}
          </Button>
        ) : null}
      </div>

      {showAddForm ? (
        <AddressEditor
          commerceId={commerceId}
          onSaved={() => {
            setShowAddForm(false);
            onChanged();
          }}
          onCancel={() => {
            setShowAddForm(false);
          }}
        />
      ) : null}

      {addresses.length === 0 && !showAddForm ? (
        <Card>
          <p className="text-sm text-muted-foreground">{t('commerces.addresses.empty')}</p>
        </Card>
      ) : null}

      {addresses.map((address) =>
        editingId === address.id ? (
          <AddressEditor
            key={address.id}
            commerceId={commerceId}
            initial={address}
            onSaved={() => {
              setEditingId(null);
              onChanged();
            }}
            onCancel={() => {
              setEditingId(null);
            }}
          />
        ) : (
          <Card key={address.id} className="space-y-3">
            <div className="flex flex-wrap items-start justify-between gap-3">
              <div>
                <p className="text-sm font-medium text-foreground">
                  {translateAddressType(t, address.addressType)}
                  {address.isPrimary ? ` · ${t('commerces.addresses.primaryBadge')}` : ''}
                </p>
                <p className="text-sm text-muted-foreground">
                  {address.street}, {address.number}
                  {address.district ? ` — ${address.district}` : ''}
                </p>
                <p className="text-sm text-muted-foreground">
                  {address.city}/{address.state} · {address.postalCode}
                </p>
              </div>
              <Button
                type="button"
                variant="secondary"
                onClick={() => {
                  setEditingId(address.id);
                  setShowAddForm(false);
                }}
              >
                {t('common.edit')}
              </Button>
            </div>
          </Card>
        ),
      )}
    </div>
  );
}
