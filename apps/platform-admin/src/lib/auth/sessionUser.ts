const EMAIL_KEY = 'full_sales_platform_user_email';

export function getStoredUserEmail(): string | null {
  return sessionStorage.getItem(EMAIL_KEY);
}

export function setStoredUserEmail(email: string): void {
  sessionStorage.setItem(EMAIL_KEY, email);
}

export function clearStoredUserEmail(): void {
  sessionStorage.removeItem(EMAIL_KEY);
}

const IMPERSONATION_KEY = 'full_sales_platform_impersonation';

export type StoredImpersonation = {
  tenantId: string;
  grantId?: string;
};

export function setStoredImpersonation(value: StoredImpersonation | null): void {
  if (!value) {
    sessionStorage.removeItem(IMPERSONATION_KEY);
    return;
  }
  sessionStorage.setItem(IMPERSONATION_KEY, JSON.stringify(value));
}

export function getStoredImpersonation(): StoredImpersonation | null {
  const raw = sessionStorage.getItem(IMPERSONATION_KEY);
  if (!raw) {
    return null;
  }
  try {
    return JSON.parse(raw) as StoredImpersonation;
  } catch {
    return null;
  }
}

export function clearStoredImpersonation(): void {
  sessionStorage.removeItem(IMPERSONATION_KEY);
}
