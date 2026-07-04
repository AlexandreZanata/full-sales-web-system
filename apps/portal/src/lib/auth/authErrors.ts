export type LoginErrorCode = 'INVALID_CREDENTIALS' | 'RATE_LIMITED' | 'NOT_PORTAL_USER' | 'UNKNOWN';

export class PortalLoginError extends Error {
  readonly code: LoginErrorCode;

  constructor(message: string, code: LoginErrorCode) {
    super(message);
    this.name = 'PortalLoginError';
    this.code = code;
  }
}

export function loginErrorMessage(code: LoginErrorCode): string {
  switch (code) {
    case 'INVALID_CREDENTIALS':
      return 'Invalid email or password.';
    case 'RATE_LIMITED':
      return 'Too many attempts. Please wait and try again.';
    case 'NOT_PORTAL_USER':
      return 'This account is not authorized for the commerce portal.';
    default:
      return 'Unable to sign in. Please try again.';
  }
}
