import type { ButtonHTMLAttributes, ReactNode } from 'react';

import { cn } from '@/lib/utils';

/** Solid action chips — white label; each tone is a distinct strong fill. */
export type TableActionTone = 'open' | 'primary' | 'success' | 'danger' | 'warning' | 'neutral';

const toneClasses: Record<TableActionTone, string> = {
  open: 'border border-foreground bg-foreground text-white hover:bg-foreground/90',
  primary: 'border border-sky-700 bg-sky-700 text-white hover:bg-sky-800',
  success: 'border border-status-active bg-status-active text-white hover:bg-status-active/90',
  danger: 'border border-destructive bg-destructive text-white hover:bg-destructive/90',
  warning: 'border border-amber-600 bg-amber-600 text-white hover:bg-amber-700',
  neutral: 'border border-slate-600 bg-slate-600 text-white hover:bg-slate-700',
};

const baseClass =
  'inline-flex h-8 min-h-8 items-center justify-center rounded-md px-3 text-xs font-semibold no-underline transition-colors disabled:cursor-not-allowed disabled:opacity-50';

export function tableActionClass(tone: TableActionTone = 'open', className?: string): string {
  return cn(baseClass, toneClasses[tone], className);
}

export function TableActions({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <div className={cn('flex flex-wrap items-center justify-end gap-1.5', className)}>{children}</div>
  );
}

type TableActionButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  tone?: TableActionTone;
};

export function TableActionButton({
  tone = 'open',
  className,
  type = 'button',
  ...props
}: TableActionButtonProps) {
  return <button type={type} className={tableActionClass(tone, className)} {...props} />;
}
