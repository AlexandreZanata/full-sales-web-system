import { USER_ROLES, type UserRoleOption } from '@/lib/users/constants';

export type CreateUserFormValues = {
  name: string;
  email: string;
  password: string;
  role: UserRoleOption | '';
  commerceId: string;
};

export type CreateUserFormErrors = Partial<Record<keyof CreateUserFormValues, string>>;

const EMAIL_PATTERN = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

export function validateCreateUserForm(values: CreateUserFormValues): CreateUserFormErrors {
  const errors: CreateUserFormErrors = {};
  const name = values.name.trim();
  const email = values.email.trim().toLowerCase();

  if (!name) {
    errors.name = 'Name is required';
  }

  if (!email) {
    errors.email = 'Email is required';
  } else if (!EMAIL_PATTERN.test(email)) {
    errors.email = 'Enter a valid email address';
  }

  if (!values.password) {
    errors.password = 'Password is required';
  } else if (values.password.length < 8) {
    errors.password = 'Password must be at least 8 characters';
  }

  if (!values.role || !USER_ROLES.includes(values.role)) {
    errors.role = 'Select a role';
  }

  if (values.role === 'CommerceContact' && !values.commerceId) {
    errors.commerceId = 'Select a commerce for commerce contacts';
  }

  return errors;
}

export function hasCreateUserFormErrors(errors: CreateUserFormErrors): boolean {
  return Object.keys(errors).length > 0;
}

export function toCreateUserPayload(values: CreateUserFormValues) {
  return {
    name: values.name.trim(),
    email: values.email.trim().toLowerCase(),
    password: values.password,
    role: values.role,
    ...(values.role === 'CommerceContact' && values.commerceId
      ? { commerceId: values.commerceId }
      : {}),
  };
}
