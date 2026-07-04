import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import {
  ADDRESS_TYPE_LABELS,
  ADDRESS_TYPES,
  type AddressTypeOption,
} from '@/lib/commerces/constants';
import type { AddressFormValues } from '@/lib/commerces/validation';

type AddressFormFieldsProps = {
  values: AddressFormValues;
  errors: Partial<Record<keyof AddressFormValues, string>>;
  showTypeField?: boolean;
  onChange: <K extends keyof AddressFormValues>(key: K, value: AddressFormValues[K]) => void;
};

export function AddressFormFields({
  values,
  errors,
  showTypeField = true,
  onChange,
}: AddressFormFieldsProps) {
  return (
    <div className="space-y-4">
      {showTypeField ? (
        <Select
          label="Address type"
          name="addressType"
          value={values.addressType}
          error={errors.addressType}
          onChange={(event) => {
            onChange('addressType', event.target.value as AddressTypeOption | '');
          }}
        >
          <option value="">Select type</option>
          {ADDRESS_TYPES.map((type) => (
            <option key={type} value={type}>
              {ADDRESS_TYPE_LABELS[type]}
            </option>
          ))}
        </Select>
      ) : null}

      <div className="grid gap-4 sm:grid-cols-2">
        <Input
          label="Street"
          name="street"
          value={values.street}
          error={errors.street}
          onChange={(event) => {
            onChange('street', event.target.value);
          }}
        />
        <Input
          label="Number"
          name="number"
          value={values.number}
          error={errors.number}
          onChange={(event) => {
            onChange('number', event.target.value);
          }}
        />
        <Input
          label="District"
          name="district"
          value={values.district}
          onChange={(event) => {
            onChange('district', event.target.value);
          }}
        />
        <Input
          label="City"
          name="city"
          value={values.city}
          error={errors.city}
          onChange={(event) => {
            onChange('city', event.target.value);
          }}
        />
        <Input
          label="State"
          name="state"
          maxLength={2}
          value={values.state}
          error={errors.state}
          onChange={(event) => {
            onChange('state', event.target.value.toUpperCase());
          }}
        />
        <Input
          label="Postal code"
          name="postalCode"
          inputMode="numeric"
          value={values.postalCode}
          error={errors.postalCode}
          onChange={(event) => {
            onChange('postalCode', event.target.value.replace(/\D/g, '').slice(0, 8));
          }}
        />
      </div>

      <label className="flex items-center gap-2 text-sm text-foreground">
        <input
          type="checkbox"
          checked={values.isPrimary}
          onChange={(event) => {
            onChange('isPrimary', event.target.checked);
          }}
        />
        Primary address for this type
      </label>
      <p className="text-xs text-muted-foreground">
        Only one primary Billing and one primary Delivery address are allowed per commerce.
      </p>
    </div>
  );
}
