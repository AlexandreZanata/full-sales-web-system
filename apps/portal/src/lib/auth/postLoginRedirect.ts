const POST_LOGIN_REDIRECT_KEY = 'portal.postLoginRedirect';

export function setPostLoginRedirect(path: string): void {
  sessionStorage.setItem(POST_LOGIN_REDIRECT_KEY, path);
}

export function consumePostLoginRedirect(): string | undefined {
  const value = sessionStorage.getItem(POST_LOGIN_REDIRECT_KEY);
  if (value) {
    sessionStorage.removeItem(POST_LOGIN_REDIRECT_KEY);
  }
  return value ?? undefined;
}

export function resolvePostLoginRedirect(searchRedirect?: string): string {
  return searchRedirect ?? consumePostLoginRedirect() ?? '/';
}
