import { InvalidEmailError } from '../errors/domain-error.js';

/** Validated email address for user identity fields. */
export class Email {
  private constructor(readonly value: string) {}

  static parse(value: string): Email {
    const trimmed = value.trim().toLowerCase();
    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(trimmed)) {
      throw new InvalidEmailError();
    }
    return new Email(trimmed);
  }

  toString(): string {
    return this.value;
  }
}
