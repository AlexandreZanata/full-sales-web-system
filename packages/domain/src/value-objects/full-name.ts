import { InvalidFullNameError } from '../errors/domain-error.js';

/** User display name with minimum structure validation. */
export class FullName {
  private constructor(readonly value: string) {}

  static parse(value: string): FullName {
    const trimmed = value.trim().replace(/\s+/g, ' ');
    const parts = trimmed.split(' ').filter((part) => part.length > 0);
    if (parts.length < 2 || parts.some((part) => part.length < 2)) {
      throw new InvalidFullNameError();
    }
    return new FullName(trimmed);
  }

  toString(): string {
    return this.value;
  }
}
