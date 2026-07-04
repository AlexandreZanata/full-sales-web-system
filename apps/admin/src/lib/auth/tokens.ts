const ACCESS_KEY = 'full_sales_admin_access_token';
const REFRESH_KEY = 'full_sales_admin_refresh_token';

let memoryAccessToken: string | null = null;
let memoryRefreshToken: string | null = null;

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

export type AuthTokens = {
  accessToken: string;
  refreshToken: string;
};

export function getAccessToken(): string | null {
  return memoryAccessToken ?? readStorage(ACCESS_KEY);
}

export function getRefreshToken(): string | null {
  return memoryRefreshToken ?? readStorage(REFRESH_KEY);
}

export function setTokens(tokens: AuthTokens): void {
  memoryAccessToken = tokens.accessToken;
  memoryRefreshToken = tokens.refreshToken;
  writeStorage(ACCESS_KEY, tokens.accessToken);
  writeStorage(REFRESH_KEY, tokens.refreshToken);
}

export function clearTokens(): void {
  memoryAccessToken = null;
  memoryRefreshToken = null;
  writeStorage(ACCESS_KEY, null);
  writeStorage(REFRESH_KEY, null);
}

export function hasSession(): boolean {
  return getAccessToken() !== null;
}
