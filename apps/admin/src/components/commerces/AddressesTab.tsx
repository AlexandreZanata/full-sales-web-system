import { useState, type SubmitEvent } from 'react';

import { AddressFormFields } from '@/components/commerces/AddressFormFields';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { useToast } from '@/hooks/useToast';
import { ApiError } from '@/lib/api/client';
import { createCommerceAddress, updateCommerceAddress } from '@/lib/api/commerces';
import type { CommerceAddress } from '@/lib/api/types';
import { ADDRESS_TYPE_LABELS } from '@/lib/commerces/constants';
import {
  hasFormErrors,
  toAddressPayload,
  toUpdateAddressPayload,
  validateAddressForm,
  type AddressFormValues,
} from '@/lib/commerces/validation';
import { formatApiErrorMessage } from '@/lib/utils';

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
        toast.success('Address updated');
      } else {
        await createCommerceAddress(commerceId, toAddressPayload(values));
        toast.success('Address added');
      }
      onSaved();
    } catch (error) {
      const message =
        error instanceof ApiError
          ? formatApiErrorMessage(error.message, error.code)
          : 'Unable to save address';
      toast.error(message);
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
            {submitting ? 'Saving…' : initial ? 'Save changes' : 'Add address'}
          </Button>
          {onCancel ? (
            <Button type="button" variant="secondary" onClick={onCancel}>
              Cancel
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
  const [showAddForm, setShowAddForm] = useState(false);
  const [editingId, setEditingId] = useState<string | null>(null);

  return (
    <div className="space-y-4">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <p className="text-sm text-muted-foreground">
          Manage billing and delivery addresses for this commerce.
        </p>
        {!showAddForm ? (
          <Button
            type="button"
            onClick={() => {
              setShowAddForm(true);
              setEditingId(null);
            }}
          >
            Add address
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
          <p className="text-sm text-muted-foreground">No addresses yet.</p>
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
                  {ADDRESS_TYPE_LABELS[address.addressType]}
                  {address.isPrimary ? ' · Primary' : ''}
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
                Edit
              </Button>
            </div>
          </Card>
        ),
      )}
    </div>
  );
}
