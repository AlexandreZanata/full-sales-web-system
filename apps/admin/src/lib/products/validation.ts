import { parsePriceInput } from '@/lib/products/formatPrice';
import type { CreateProductRequest } from '@/lib/api/types';

export type CreateProductFormValues = {
  name: string;
  sku: string;
  price: string;
  priceCurrency: string;
};

export type EditProductFormValues = {
  name: string;
  price: string;
  priceCurrency: string;
  category: string;
  unitOfMeasure: string;
};

export type FormErrors<T extends string> = Partial<Record<T, string>>;

export function validateCreateProductForm(
  values: CreateProductFormValues,
): FormErrors<keyof CreateProductFormValues> {
  const errors: FormErrors<keyof CreateProductFormValues> = {};

  if (!values.name.trim()) {
    errors.name = 'forms.validation.nameRequired';
  }
  if (!values.sku.trim()) {
    errors.sku = 'forms.validation.skuRequired';
  }
  if (parsePriceInput(values.price) === null) {
    errors.price = 'forms.validation.priceInvalid';
  }

  return errors;
}

export function validateEditProductForm(
  values: EditProductFormValues,
): FormErrors<keyof EditProductFormValues> {
  const errors: FormErrors<keyof EditProductFormValues> = {};

  if (!values.name.trim()) {
    errors.name = 'forms.validation.nameRequired';
  }
  if (parsePriceInput(values.price) === null) {
    errors.price = 'forms.validation.priceInvalid';
  }
  if (!values.unitOfMeasure.trim()) {
    errors.unitOfMeasure = 'forms.validation.unitOfMeasureRequired';
  }

  return errors;
}

export function hasFormErrors<T extends string>(errors: FormErrors<T>): boolean {
  return Object.keys(errors).length > 0;
}

export function toCreateProductPayload(values: CreateProductFormValues): CreateProductRequest {
  const priceAmount = parsePriceInput(values.price);
  if (priceAmount === null) {
    throw new Error('Invalid price');
  }

  return {
    name: values.name.trim(),
    sku: values.sku.trim(),
    priceAmount,
    priceCurrency: values.priceCurrency.trim() || 'BRL',
  };
}

export function toUpdateProductPayload(values: EditProductFormValues) {
  const priceAmount = parsePriceInput(values.price);
  if (priceAmount === null) {
    throw new Error('Invalid price');
  }

  return {
    name: values.name.trim(),
    priceAmount,
    priceCurrency: values.priceCurrency.trim() || 'BRL',
    category: values.category.trim() || undefined,
    unitOfMeasure: values.unitOfMeasure.trim(),
  };
}
