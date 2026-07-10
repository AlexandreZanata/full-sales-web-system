const ACCESS_KEY = 'full_sales_platform_access_token';
const REFRESH_KEY = 'full_sales_platform_refresh_token';
const EXPIRES_KEY = 'full_sales_platform_token_expires_at';

export type AuthTokens = {
  accessToken: string;
  refreshToken: string;
  expiresIn?: number;
};

export function setTokens(tokens: AuthTokens): void {
  sessionStorage.setItem(ACCESS_KEY, tokens.accessToken);
  sessionStorage.setItem(REFRESH_KEY, tokens.refreshToken);
  if (tokens.expiresIn) {
    sessionStorage.setItem(EXPIRES_KEY, String(Date.now() + tokens.expiresIn * 1000));
  }
}

export function clearTokens(): void {
  sessionStorage.removeItem(ACCESS_KEY);
  sessionStorage.removeItem(REFRESH_KEY);
  sessionStorage.removeItem(EXPIRES_KEY);
}

export function getAccessToken(): string | null {
  return sessionStorage.getItem(ACCESS_KEY);
}

export function getRefreshToken(): string | null {
  return sessionStorage.getItem(REFRESH_KEY);
}

export function getTokenExpiresAt(): number | null {
  const raw = sessionStorage.getItem(EXPIRES_KEY);
  return raw ? Number(raw) : null;
}

export function hasSession(): boolean {
  return Boolean(getAccessToken() && getRefreshToken());
}

const REFRESH_LEAD_MS = 60_000;

export function msUntilTokenRefresh(expiresAtMs: number, nowMs = Date.now()): number {
  const target = expiresAtMs - REFRESH_LEAD_MS - nowMs;
  return Math.max(target, 0);
}
