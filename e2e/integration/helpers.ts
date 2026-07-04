const API_BASE = process.env.INTEGRATION_API_URL ?? 'http://127.0.0.1:8080';

export async function isApiReachable(): Promise<boolean> {
  try {
    const response = await fetch(`${API_BASE}/health`, { signal: AbortSignal.timeout(2_000) });
    return response.ok;
  } catch {
    return false;
  }
}

export async function login(email: string, password: string): Promise<string> {
  const response = await fetch(`${API_BASE}/v1/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', Accept: 'application/json' },
    body: JSON.stringify({ email, password }),
  });
  if (!response.ok) {
    throw new Error(`Login failed for ${email}: ${response.status}`);
  }
  const body = (await response.json()) as { accessToken: string };
  return body.accessToken;
}

export { API_BASE };
