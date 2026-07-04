export type SiteIdentityFormValues = {
  displayName: string;
};

export type SiteIdentityFormErrors = Partial<Record<keyof SiteIdentityFormValues, string>>;

export function validateSiteIdentityForm(values: SiteIdentityFormValues): SiteIdentityFormErrors {
  const errors: SiteIdentityFormErrors = {};
  const trimmed = values.displayName.trim();

  if (!trimmed) {
    errors.displayName = 'forms.validation.nameRequired';
  } else if (trimmed.length > 200) {
    errors.displayName = 'settings.validation.displayNameTooLong';
  }

  return errors;
}

export function hasSiteIdentityErrors(errors: SiteIdentityFormErrors): boolean {
  return Object.keys(errors).length > 0;
}
