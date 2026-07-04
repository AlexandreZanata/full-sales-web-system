export type LoginErrorCode = 'INVALID_CREDENTIALS' | 'RATE_LIMITED' | 'NOT_FIELD_USER' | 'UNKNOWN';

export class FieldLoginError extends Error {
  readonly code: LoginErrorCode;

  constructor(message: string, code: LoginErrorCode) {
    super(message);
    this.name = 'FieldLoginError';
    this.code = code;
  }
}

export function loginErrorMessage(code: LoginErrorCode): string {
  switch (code) {
    case 'INVALID_CREDENTIALS':
      return 'E-mail ou senha inválidos.';
    case 'RATE_LIMITED':
      return 'Muitas tentativas. Aguarde e tente novamente.';
    case 'NOT_FIELD_USER':
      return 'Esta conta não está autorizada para o app de campo.';
    default:
      return 'Não foi possível entrar. Tente novamente.';
  }
}
