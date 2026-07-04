import { InvalidCnpjError } from '../errors/domain-error.js';
import { isValidCnpj } from './cnpj-validate.js';

/** Brazilian company tax identifier with check-digit validation. */
export class Cnpj {
  private constructor(private readonly digits: string) {}

  static parse(raw: string): Cnpj {
    const normalized = raw.replace(/\D/g, '');
    if (!isValidCnpj(normalized)) {
      throw new InvalidCnpjError();
    }
    return new Cnpj(normalized);
  }

  toString(): string {
    return this.digits;
  }

  formatted(): string {
    const d = this.digits;
    return `${d.slice(0, 2)}.${d.slice(2, 5)}.${d.slice(5, 8)}/${d.slice(8, 12)}-${d.slice(12)}`;
  }
}
