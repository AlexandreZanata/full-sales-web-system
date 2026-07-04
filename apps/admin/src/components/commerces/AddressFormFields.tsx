import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { ADDRESS_TYPES, type AddressTypeOption } from '@/lib/commerces/constants';
import type { AddressFormValues } from '@/lib/commerces/validation';
import { useI18n } from '@/lib/i18n/context';
import { translateAddressType, translateFormError } from '@/lib/i18n/labels';

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
  const { t } = useI18n();

  return (
    <div className="space-y-4">
      {showTypeField ? (
        <Select
          label={t('forms.fields.addressType')}
          name="addressType"
          value={values.addressType}
          error={translateFormError(t, errors.addressType)}
          onChange={(event) => {
            onChange('addressType', event.target.value as AddressTypeOption | '');
          }}
        >
          <option value="">{t('forms.placeholders.selectAddressType')}</option>
          {ADDRESS_TYPES.map((type) => (
            <option key={type} value={type}>
              {translateAddressType(t, type)}
            </option>
          ))}
        </Select>
      ) : null}

      <div className="grid gap-4 sm:grid-cols-2">
        <Input
          label={t('forms.fields.street')}
          name="street"
          value={values.street}
          error={translateFormError(t, errors.street)}
          onChange={(event) => {
            onChange('street', event.target.value);
          }}
        />
        <Input
          label={t('forms.fields.number')}
          name="number"
          value={values.number}
          error={translateFormError(t, errors.number)}
          onChange={(event) => {
            onChange('number', event.target.value);
          }}
        />
        <Input
          label={t('forms.fields.district')}
          name="district"
          value={values.district}
          onChange={(event) => {
            onChange('district', event.target.value);
          }}
        />
        <Input
          label={t('forms.fields.city')}
          name="city"
          value={values.city}
          error={translateFormError(t, errors.city)}
          onChange={(event) => {
            onChange('city', event.target.value);
          }}
        />
        <Input
          label={t('forms.fields.state')}
          name="state"
          maxLength={2}
          value={values.state}
          error={translateFormError(t, errors.state)}
          onChange={(event) => {
            onChange('state', event.target.value.toUpperCase());
          }}
        />
        <Input
          label={t('forms.fields.postalCode')}
          name="postalCode"
          inputMode="numeric"
          value={values.postalCode}
          error={translateFormError(t, errors.postalCode)}
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
        {t('commerces.addresses.primaryCheckbox')}
      </label>
      <p className="text-xs text-muted-foreground">{t('commerces.addresses.primaryConstraint')}</p>
    </div>
  );
}
