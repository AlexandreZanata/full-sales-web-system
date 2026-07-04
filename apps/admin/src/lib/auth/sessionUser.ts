const EMAIL_KEY = 'full_sales_admin_user_email';

let memoryEmail: string | null = null;

function readStorage(key: string): string | null {
  try {
    return sessionStorage.getItem(key);
  } catch {
    return null;
  }
}

function writeStorage(key: string, value: string | null): void {
  try {
    if (value === null) {
      sessionStorage.removeItem(key);
    } else {
      sessionStorage.setItem(key, value);
    }
  } catch {
    // sessionStorage unavailable
  }
}

export function getStoredUserEmail(): string | null {
  return memoryEmail ?? readStorage(EMAIL_KEY);
}

export function setStoredUserEmail(email: string): void {
  memoryEmail = email;
  writeStorage(EMAIL_KEY, email);
}

export function clearStoredUserEmail(): void {
  memoryEmail = null;
  writeStorage(EMAIL_KEY, null);
}
