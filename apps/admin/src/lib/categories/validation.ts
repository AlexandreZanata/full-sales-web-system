import type { CreateCategoryRequest, UpdateCategoryRequest } from '@/lib/api/types';

export type CategoryFormValues = {
  name: string;
  description: string;
  sortOrder: string;
  active: boolean;
};

export type CategoryFormErrors = Partial<Record<keyof CategoryFormValues, string>>;

export function validateCategoryForm(values: CategoryFormValues): CategoryFormErrors {
  const errors: CategoryFormErrors = {};

  if (!values.name.trim()) {
    errors.name = 'forms.validation.nameRequired';
  }

  if (values.sortOrder.trim()) {
    const parsed = Number.parseInt(values.sortOrder, 10);
    if (Number.isNaN(parsed) || parsed < 0) {
      errors.sortOrder = 'categories.validation.sortOrderInvalid';
    }
  }

  return errors;
}

export function hasCategoryFormErrors(errors: CategoryFormErrors): boolean {
  return Object.keys(errors).length > 0;
}

export function toCreateCategoryPayload(values: CategoryFormValues): CreateCategoryRequest {
  const payload: CreateCategoryRequest = {
    name: values.name.trim(),
    active: values.active,
  };

  const description = values.description.trim();
  if (description) {
    payload.description = description;
  }

  if (values.sortOrder.trim()) {
    payload.sortOrder = Number.parseInt(values.sortOrder, 10);
  }

  return payload;
}

export function toUpdateCategoryPayload(values: CategoryFormValues): UpdateCategoryRequest {
  const payload: UpdateCategoryRequest = {
    name: values.name.trim(),
    active: values.active,
  };

  const description = values.description.trim();
  payload.description = description || undefined;

  if (values.sortOrder.trim()) {
    payload.sortOrder = Number.parseInt(values.sortOrder, 10);
  }

  return payload;
}

export function categoryToFormValues(category: {
  name: string;
  description?: string;
  sortOrder: number;
  active: boolean;
}): CategoryFormValues {
  return {
    name: category.name,
    description: category.description ?? '',
    sortOrder: String(category.sortOrder),
    active: category.active,
  };
}
