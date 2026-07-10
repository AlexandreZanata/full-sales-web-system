import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatApiErrorMessage(message: string | undefined, fallback: string): string {
  return message && message.length > 0 ? message : fallback;
}
