export type AdminLoginErrorCode = 'INVALID_CREDENTIALS' | 'RATE_LIMITED' | 'NOT_ADMIN' | 'UNKNOWN';

export class AdminLoginError extends Error {
  readonly code: AdminLoginErrorCode;

  constructor(message: string, code: AdminLoginErrorCode) {
    super(message);
    this.name = 'AdminLoginError';
    this.code = code;
  }
}

export function loginErrorMessage(code: AdminLoginErrorCode): string {
  switch (code) {
    case 'INVALID_CREDENTIALS':
      return 'Invalid email or password.';
    case 'RATE_LIMITED':
      return 'Too many login attempts. Please try again later.';
    case 'NOT_ADMIN':
      return 'Admin access required. Your account does not have the Admin role.';
    default:
      return 'Unable to sign in. Please try again.';
  }
}
