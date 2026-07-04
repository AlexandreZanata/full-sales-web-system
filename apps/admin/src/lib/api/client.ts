import {
  clearTokens,
  getAccessToken,
  getRefreshToken,
  setTokens,
  type AuthTokens,
} from '@/lib/auth/tokens';

const DEFAULT_API_BASE = '/v1';

export type ApiErrorBody = {
  error: {
    code: string;
    message: string;
    correlationId?: string;
  };
};

export class ApiError extends Error {
  readonly status: number;
  readonly code: string;
  readonly correlationId?: string;

  constructor(status: number, body: ApiErrorBody) {
    super(body.error.message);
    this.name = 'ApiError';
    this.status = status;
    this.code = body.error.code;
    this.correlationId = body.error.correlationId;
  }
}

export type ApiRequestOptions = RequestInit & {
  skipAuth?: boolean;
  skipRefreshRetry?: boolean;
};

let refreshPromise: Promise<boolean> | null = null;
let onSessionExpired: (() => void) | null = null;

export function setSessionExpiredHandler(handler: (() => void) | null): void {
  onSessionExpired = handler;
}

export function getApiBaseUrl(): string {
  const configured = import.meta.env.VITE_API_BASE_URL as string | undefined;
  return configured?.replace(/\/$/, '') ?? DEFAULT_API_BASE;
}

function apiUrl(path: string): string {
  const normalized = path.startsWith('/') ? path : `/${path}`;
  const base = getApiBaseUrl();
  if (normalized.startsWith(base)) {
    return normalized;
  }
  return `${base}${normalized}`;
}

function buildHeaders(init?: RequestInit, accessToken?: string | null): HeadersInit {
  const headers: Record<string, string> = {
    Accept: 'application/json',
    ...(init?.headers as Record<string, string> | undefined),
  };

  if (accessToken) {
    headers.Authorization = `Bearer ${accessToken}`;
  }

  return headers;
}

export async function parseApiErrorBody(response: Response): Promise<ApiErrorBody> {
  try {
    const body = (await response.json()) as Partial<ApiErrorBody>;
    if (body.error?.message) {
      return {
        error: {
          code: body.error.code || 'UNKNOWN_ERROR',
          message: body.error.message,
          correlationId: body.error.correlationId,
        },
      };
    }
  } catch {
    // fall through
  }

  return {
    error: {
      code: 'HTTP_ERROR',
      message: response.statusText || `Request failed (${String(response.status)})`,
    },
  };
}

async function throwApiError(response: Response): Promise<never> {
  const body = await parseApiErrorBody(response);
  throw new ApiError(response.status, body);
}

export async function tryRefreshTokens(): Promise<boolean> {
  if (refreshPromise) {
    return refreshPromise;
  }

  const refreshToken = getRefreshToken();
  if (!refreshToken) {
    return false;
  }

  refreshPromise = (async () => {
    try {
      const response = await fetch(apiUrl('/auth/refresh'), {
        method: 'POST',
        headers: { Accept: 'application/json', 'Content-Type': 'application/json' },
        body: JSON.stringify({ refreshToken }),
      });

      if (!response.ok) {
        clearTokens();
        onSessionExpired?.();
        return false;
      }

      const data = (await response.json()) as AuthTokens & { expiresIn?: number };
      setTokens({
        accessToken: data.accessToken,
        refreshToken: data.refreshToken,
        expiresIn: data.expiresIn,
      });
      return true;
    } catch {
      clearTokens();
      onSessionExpired?.();
      return false;
    } finally {
      refreshPromise = null;
    }
  })();

  return refreshPromise;
}

export async function apiRequest(path: string, init?: ApiRequestOptions): Promise<Response> {
  const { skipAuth = false, skipRefreshRetry = false, ...requestInit } = init ?? {};
  const accessToken = skipAuth ? null : getAccessToken();

  let response = await fetch(apiUrl(path), {
    ...requestInit,
    headers: buildHeaders(requestInit, accessToken),
  });

  if (
    response.status === 401 &&
    !skipAuth &&
    !skipRefreshRetry &&
    path !== '/auth/refresh' &&
    path !== '/auth/login'
  ) {
    const refreshed = await tryRefreshTokens();
    if (refreshed) {
      response = await fetch(apiUrl(path), {
        ...requestInit,
        headers: buildHeaders(requestInit, getAccessToken()),
      });
    }
  }

  return response;
}

export async function apiFetch<T>(path: string, init?: ApiRequestOptions): Promise<T> {
  const response = await apiRequest(path, init);

  if (!response.ok) {
    await throwApiError(response);
  }

  if (response.status === 204) {
    return undefined as T;
  }

  return (await response.json()) as T;
}

export async function apiPost<T>(path: string, body: object, init?: ApiRequestOptions): Promise<T> {
  return apiFetch<T>(path, {
    ...init,
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...(init?.headers as Record<string, string> | undefined),
    },
    body: JSON.stringify(body),
  });
}

export async function apiPut<T>(path: string, body: object, init?: ApiRequestOptions): Promise<T> {
  return apiFetch<T>(path, {
    ...init,
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
      ...(init?.headers as Record<string, string> | undefined),
    },
    body: JSON.stringify(body),
  });
}

export async function apiPatch<T>(path: string, init?: ApiRequestOptions): Promise<T> {
  return apiFetch<T>(path, {
    ...init,
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      ...(init?.headers as Record<string, string> | undefined),
    },
  });
}

export function isUnauthorizedError(error: unknown): boolean {
  return error instanceof ApiError && error.status === 401;
}

export function isValidationError(error: unknown): boolean {
  return error instanceof ApiError && (error.status === 400 || error.status === 422);
}
