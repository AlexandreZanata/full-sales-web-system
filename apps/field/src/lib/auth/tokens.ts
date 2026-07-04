import { clearStoredUserEmail } from '@/lib/auth/sessionUser';

const ACCESS_KEY = 'full_sales_field_access_token';
const REFRESH_KEY = 'full_sales_field_refresh_token';
const EXPIRES_AT_KEY = 'full_sales_field_token_expires_at';

let memoryAccessToken: string | null = null;
let memoryRefreshToken: string | null = null;
let memoryExpiresAt: number | null = null;

function readStorage(key: string): string | null {
  try {
    return sessionStorage.getItem(key);
  } catch {
    return null;
  }
}

function writeStorage(key: string, value: string | null): void {
  try {
    if (value === null) sessionStorage.removeItem(key);
    else sessionStorage.setItem(key, value);
  } catch {
    // ignore
  }
}

export type AuthTokens = {
  accessToken: string;
  refreshToken: string;
  expiresIn?: number;
};

export function getAccessToken(): string | null {
  return memoryAccessToken ?? readStorage(ACCESS_KEY);
}

export function getRefreshToken(): string | null {
  return memoryRefreshToken ?? readStorage(REFRESH_KEY);
}

export function getTokenExpiresAt(): number | null {
  if (memoryExpiresAt !== null) return memoryExpiresAt;
  const stored = readStorage(EXPIRES_AT_KEY);
  if (!stored) return null;
  const parsed = Number(stored);
  return Number.isFinite(parsed) ? parsed : null;
}

export function setTokens(tokens: AuthTokens): void {
  memoryAccessToken = tokens.accessToken;
  memoryRefreshToken = tokens.refreshToken;
  writeStorage(ACCESS_KEY, tokens.accessToken);
  writeStorage(REFRESH_KEY, tokens.refreshToken);
  if (tokens.expiresIn !== undefined) {
    const expiresAt = Date.now() + tokens.expiresIn * 1000;
    memoryExpiresAt = expiresAt;
    writeStorage(EXPIRES_AT_KEY, String(expiresAt));
  }
}

export function clearTokens(): void {
  memoryAccessToken = null;
  memoryRefreshToken = null;
  memoryExpiresAt = null;
  writeStorage(ACCESS_KEY, null);
  writeStorage(REFRESH_KEY, null);
  writeStorage(EXPIRES_AT_KEY, null);
  clearStoredUserEmail();
}

export function hasSession(): boolean {
  return getAccessToken() !== null;
}

export function msUntilTokenRefresh(expiresAt: number, leadMs = 60_000): number {
  return Math.max(0, expiresAt - leadMs - Date.now());
}
