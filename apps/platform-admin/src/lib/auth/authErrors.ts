export type PlatformLoginErrorCode =
  'INVALID_CREDENTIALS' | 'NOT_PLATFORM_ADMIN' | 'RATE_LIMITED' | 'MFA_INVALID' | 'UNKNOWN';

export class PlatformLoginError extends Error {
  readonly code: PlatformLoginErrorCode;

  constructor(message: string, code: PlatformLoginErrorCode) {
    super(message);
    this.name = 'PlatformLoginError';
    this.code = code;
  }
}

const MESSAGES: Record<PlatformLoginErrorCode, string> = {
  INVALID_CREDENTIALS: 'Invalid email or password.',
  NOT_PLATFORM_ADMIN: 'This account is not authorized for platform administration.',
  RATE_LIMITED: 'Too many attempts. Try again later.',
  MFA_INVALID: 'Invalid verification code.',
  UNKNOWN: 'Unable to sign in. Please try again.',
};

export function loginErrorMessage(code: PlatformLoginErrorCode): string {
  return MESSAGES[code];
}
