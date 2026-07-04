import { ADDRESS_TYPES, type AddressTypeOption } from '@/lib/commerces/constants';
import { formatCnpj, isValidCnpj, stripCnpj } from '@/lib/commerces/cnpj';
import type {
  CommerceAddressRequest,
  CreateCommerceRequest,
  UpdateCommerceAddressRequest,
} from '@/lib/api/types';

export type CreateCommerceFormValues = {
  cnpj: string;
  legalName: string;
  tradeName: string;
  street: string;
  number: string;
  district: string;
  city: string;
  state: string;
  postalCode: string;
  contactPhone: string;
  contactEmail: string;
};

export type AddressFormValues = {
  addressType: AddressTypeOption | '';
  street: string;
  number: string;
  district: string;
  city: string;
  state: string;
  postalCode: string;
  isPrimary: boolean;
};

export type FormErrors<T extends string> = Partial<Record<T, string>>;

const EMAIL_PATTERN = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

export function validateCreateCommerceForm(
  values: CreateCommerceFormValues,
): FormErrors<keyof CreateCommerceFormValues> {
  const errors: FormErrors<keyof CreateCommerceFormValues> = {};
  const cnpjDigits = stripCnpj(values.cnpj);

  if (!cnpjDigits) {
    errors.cnpj = 'forms.validation.cnpjRequired';
  } else if (!isValidCnpj(cnpjDigits)) {
    errors.cnpj = 'forms.validation.cnpjInvalid';
  }

  if (!values.legalName.trim()) {
    errors.legalName = 'forms.validation.legalNameRequired';
  }

  if (!values.street.trim()) {
    errors.street = 'forms.validation.streetRequired';
  }
  if (!values.number.trim()) {
    errors.number = 'forms.validation.numberRequired';
  }
  if (!values.city.trim()) {
    errors.city = 'forms.validation.cityRequired';
  }
  if (!values.state.trim()) {
    errors.state = 'forms.validation.stateRequired';
  } else if (values.state.trim().length !== 2) {
    errors.state = 'forms.validation.stateInvalid';
  }
  if (!values.postalCode.trim()) {
    errors.postalCode = 'forms.validation.postalCodeRequired';
  }

  const email = values.contactEmail.trim();
  if (email && !EMAIL_PATTERN.test(email)) {
    errors.contactEmail = 'forms.validation.emailInvalid';
  }

  return errors;
}

export function hasFormErrors<T extends string>(errors: FormErrors<T>): boolean {
  return Object.keys(errors).length > 0;
}

export function toCreateCommercePayload(values: CreateCommerceFormValues): CreateCommerceRequest {
  return {
    cnpj: stripCnpj(values.cnpj),
    legalName: values.legalName.trim(),
    ...(values.tradeName.trim() ? { tradeName: values.tradeName.trim() } : {}),
    address: {
      street: values.street.trim(),
      number: values.number.trim(),
      ...(values.district.trim() ? { district: values.district.trim() } : {}),
      city: values.city.trim(),
      state: values.state.trim().toUpperCase(),
      postalCode: stripCnpj(values.postalCode),
    },
    contact: {
      ...(values.contactPhone.trim() ? { phone: values.contactPhone.trim() } : {}),
      ...(values.contactEmail.trim() ? { email: values.contactEmail.trim().toLowerCase() } : {}),
    },
  };
}

export function validateAddressForm(
  values: AddressFormValues,
): FormErrors<keyof AddressFormValues> {
  const errors: FormErrors<keyof AddressFormValues> = {};

  if (!values.addressType || !ADDRESS_TYPES.includes(values.addressType)) {
    errors.addressType = 'forms.validation.addressTypeRequired';
  }
  if (!values.street.trim()) {
    errors.street = 'forms.validation.streetRequired';
  }
  if (!values.number.trim()) {
    errors.number = 'forms.validation.numberRequired';
  }
  if (!values.city.trim()) {
    errors.city = 'forms.validation.cityRequired';
  }
  if (!values.state.trim()) {
    errors.state = 'forms.validation.stateRequired';
  } else if (values.state.trim().length !== 2) {
    errors.state = 'forms.validation.stateInvalid';
  }
  if (!values.postalCode.trim()) {
    errors.postalCode = 'forms.validation.postalCodeRequired';
  }

  return errors;
}

export function toAddressPayload(values: AddressFormValues): CommerceAddressRequest {
  return {
    addressType: values.addressType as AddressTypeOption,
    street: values.street.trim(),
    number: values.number.trim(),
    ...(values.district.trim() ? { district: values.district.trim() } : {}),
    city: values.city.trim(),
    state: values.state.trim().toUpperCase(),
    postalCode: stripCnpj(values.postalCode),
    isPrimary: values.isPrimary,
  };
}

export function toUpdateAddressPayload(values: AddressFormValues): UpdateCommerceAddressRequest {
  return {
    street: values.street.trim(),
    number: values.number.trim(),
    ...(values.district.trim() ? { district: values.district.trim() } : {}),
    city: values.city.trim(),
    state: values.state.trim().toUpperCase(),
    postalCode: stripCnpj(values.postalCode),
    isPrimary: values.isPrimary,
  };
}

export function formatCnpjInput(raw: string): string {
  return formatCnpj(raw);
}
