/** Base class for domain-layer failures. */
export class DomainError extends Error {
  constructor(message: string) {
    super(message);
    this.name = new.target.name;
  }
}

export class NegativeMoneyAmountError extends DomainError {
  constructor() {
    super('money amount cannot be negative');
  }
}

export class InvalidCurrencyError extends DomainError {
  constructor() {
    super('invalid currency code: must be 3 uppercase ASCII letters');
  }
}

export class CurrencyMismatchError extends DomainError {
  constructor(
    public readonly left: string,
    public readonly right: string,
  ) {
    super(`currency mismatch: ${left} vs ${right}`);
  }
}

export class MoneyOverflowError extends DomainError {
  constructor() {
    super('money amount overflow');
  }
}

export class InvalidQuantityError extends DomainError {
  constructor() {
    super('quantity must be a positive integer');
  }
}

export class InvalidUuidError extends DomainError {
  constructor(label: string) {
    super(`invalid ${label}`);
  }
}

export class InvalidCnpjError extends DomainError {
  constructor() {
    super('invalid CNPJ check digits');
  }
}

export class InvalidSkuError extends DomainError {
  constructor() {
    super('sku must be a non-empty alphanumeric identifier');
  }
}

export class InvalidEmailError extends DomainError {
  constructor() {
    super('invalid email address');
  }
}

export class InvalidFullNameError extends DomainError {
  constructor() {
    super('full name must contain at least two non-empty parts');
  }
}

export class InactiveProductError extends DomainError {
  constructor() {
    super('inactive product cannot be added to sale');
  }
}

export class InactiveCommerceError extends DomainError {
  constructor() {
    super('inactive commerce cannot be referenced in new sale');
  }
}

export class EmptySaleError extends DomainError {
  constructor() {
    super('cannot confirm sale without items');
  }
}

export class InvalidSaleTransitionError extends DomainError {
  constructor(
    public readonly from: string,
    public readonly to: string,
  ) {
    super(`invalid sale transition: ${from} → ${to}`);
  }
}
