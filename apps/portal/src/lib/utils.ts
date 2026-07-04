import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatApiErrorMessage(message: string | undefined, fallback: string): string {
  return message && message.length > 0 ? message : fallback;
}

export function randomId(): string {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return crypto.randomUUID();
  }
  return `id-${String(Date.now())}-${Math.random().toString(36).slice(2, 9)}`;
}
