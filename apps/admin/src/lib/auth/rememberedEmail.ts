const STORAGE_KEY = 'admin.login.rememberedEmail';

export function readRememberedEmail(): string | null {
  try {
    const value = localStorage.getItem(STORAGE_KEY);
    return value && value.includes('@') ? value : null;
  } catch {
    return null;
  }
}

export function writeRememberedEmail(email: string | null): void {
  try {
    if (email) {
      localStorage.setItem(STORAGE_KEY, email.trim());
    } else {
      localStorage.removeItem(STORAGE_KEY);
    }
  } catch {
    // Ignore quota / private-mode failures.
  }
}
