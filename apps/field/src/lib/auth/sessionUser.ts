const EMAIL_KEY = 'full_sales_field_user_email';

export function getStoredUserEmail(): string | null {
  try {
    return sessionStorage.getItem(EMAIL_KEY);
  } catch {
    return null;
  }
}

export function setStoredUserEmail(email: string): void {
  try {
    sessionStorage.setItem(EMAIL_KEY, email);
  } catch {
    // ignore
  }
}

export function clearStoredUserEmail(): void {
  try {
    sessionStorage.removeItem(EMAIL_KEY);
  } catch {
    // ignore
  }
}
