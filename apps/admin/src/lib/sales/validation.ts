import type { CreateSaleRequest } from '@/lib/api/types';
import { PAYMENT_METHODS } from '@/lib/sales/constants';

export type SaleLineFormValues = {
  productId: string;
  quantity: string;
};

export type CreateSaleFormValues = {
  commerceId: string;
  driverId: string;
  paymentMethod: (typeof PAYMENT_METHODS)[number] | '';
  items: SaleLineFormValues[];
};

export type CreateSaleFormErrors = Partial<
  Record<'commerceId' | 'driverId' | 'paymentMethod' | 'itemsRoot', string> & {
    items: Partial<Record<keyof SaleLineFormValues, string>>[];
  }
>;

function parseQuantity(raw: string): number | null {
  const qty = Number.parseInt(raw, 10);
  if (!raw.trim() || !Number.isFinite(qty) || qty <= 0) {
    return null;
  }
  return qty;
}

export function validateCreateSaleForm(values: CreateSaleFormValues): CreateSaleFormErrors {
  const errors: CreateSaleFormErrors = {};

  if (!values.commerceId) {
    errors.commerceId = 'forms.validation.selectCommerce';
  }

  if (!values.driverId) {
    errors.driverId = 'forms.validation.selectDriver';
  }

  if (!values.paymentMethod || !PAYMENT_METHODS.includes(values.paymentMethod)) {
    errors.paymentMethod = 'forms.validation.selectPaymentMethod';
  }

  const itemErrors: CreateSaleFormErrors['items'] = [];
  let completeLines = 0;

  values.items.forEach((line, index) => {
    const lineErrors: Partial<Record<keyof SaleLineFormValues, string>> = {};
    const qty = parseQuantity(line.quantity);

    if (!line.productId) {
      lineErrors.productId = 'forms.validation.selectProduct';
    }
    if (qty === null) {
      lineErrors.quantity = 'forms.validation.quantityNonZero';
    }
    if (line.productId && qty !== null) {
      completeLines += 1;
    }
    itemErrors[index] = lineErrors;
  });

  if (completeLines === 0) {
    errors.itemsRoot = 'forms.validation.itemsRequired';
    errors.items = itemErrors;
  } else if (itemErrors.some((line) => Object.keys(line).length > 0)) {
    errors.items = itemErrors;
  }

  return errors;
}

export function hasFormErrors(errors: CreateSaleFormErrors): boolean {
  if (errors.commerceId || errors.driverId || errors.paymentMethod || errors.itemsRoot) {
    return true;
  }
  return Boolean(errors.items?.some((line) => Object.keys(line).length > 0));
}

export function toCreateSalePayload(values: CreateSaleFormValues): CreateSaleRequest {
  const paymentMethod = values.paymentMethod;
  if (!paymentMethod) {
    throw new Error('paymentMethod is required');
  }
  return {
    commerceId: values.commerceId,
    driverId: values.driverId,
    paymentMethod,
    items: values.items
      .filter((line) => line.productId && parseQuantity(line.quantity) !== null)
      .map((line) => ({
        productId: line.productId,
        quantity: parseQuantity(line.quantity) ?? 0,
      })),
  };
}
