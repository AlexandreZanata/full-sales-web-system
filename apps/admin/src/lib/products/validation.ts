import { parsePriceInput } from '@/lib/products/formatPrice';
import type { CreateProductRequest } from '@/lib/api/types';

const MAX_DESCRIPTION_LENGTH = 2_000;

export type CreateProductFormValues = {
  name: string;
  sku: string;
  price: string;
  priceCurrency: string;
  categoryId: string;
  description: string;
};

export type EditProductFormValues = {
  name: string;
  price: string;
  priceCurrency: string;
  unitOfMeasure: string;
  categoryId: string;
  description: string;
};

export type FormErrors<T extends string> = Partial<Record<T, string>>;

function validateDescription(description: string): string | undefined {
  if (description.trim().length > MAX_DESCRIPTION_LENGTH) {
    return 'forms.validation.descriptionMax';
  }
  return undefined;
}

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
  const descriptionError = validateDescription(values.description);
  if (descriptionError) {
    errors.description = descriptionError;
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
  const descriptionError = validateDescription(values.description);
  if (descriptionError) {
    errors.description = descriptionError;
  }

  return errors;
}

export function hasFormErrors<T extends string>(errors: FormErrors<T>): boolean {
  return Object.keys(errors).length > 0;
}

function normalizeDescription(description: string): string | undefined {
  const trimmed = description.trim();
  return trimmed.length > 0 ? trimmed : undefined;
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
    ...(values.categoryId ? { categoryId: values.categoryId } : {}),
    ...(normalizeDescription(values.description)
      ? { description: normalizeDescription(values.description) }
      : {}),
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
    unitOfMeasure: values.unitOfMeasure.trim(),
    categoryId: values.categoryId || null,
    description: normalizeDescription(values.description) ?? null,
  };
}
