import { SaleStatus } from '@full-sales/domain';

/**
 * RFC 9457 / API-CONTRACT.md error shape for client-side handling.
 */
export interface ApplicationErrorBody {
  error: {
    code: string;
    message: string;
    correlationId: string;
  };
}

export class ApplicationError extends Error {
  readonly code: string;
  readonly correlationId: string;

  constructor(code: string, message: string, correlationId: string) {
    super(message);
    this.name = 'ApplicationError';
    this.code = code;
    this.correlationId = correlationId;
  }

  toJSON(): ApplicationErrorBody {
    return {
      error: {
        code: this.code,
        message: this.message,
        correlationId: this.correlationId,
      },
    };
  }
}

/** Ensures application layer depends on domain package only (scaffold check). */
export function applicationDomainRef(): typeof SaleStatus.Pending {
  return SaleStatus.Pending;
}
