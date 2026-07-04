import { cn } from '@/lib/utils';
import { useId, type SelectHTMLAttributes } from 'react';

type SelectProps = SelectHTMLAttributes<HTMLSelectElement> & {
  label?: string;
  error?: string;
};

export function Select({ className, label, error, id, children, ...props }: SelectProps) {
  const generatedId = useId();
  const inputId = id ?? props.name ?? generatedId;

  return (
    <div className="space-y-1.5">
      {label ? (
        <label
          htmlFor={inputId}
          className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground"
        >
          {label}
        </label>
      ) : null}
      <select
        id={inputId}
        className={cn(
          'h-10 w-full rounded-md border bg-surface px-3 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-admin-accent',
          error ? 'border-destructive focus:ring-destructive' : 'border-input',
          className,
        )}
        aria-invalid={Boolean(error)}
        {...props}
      >
        {children}
      </select>
      {error ? <p className="text-xs text-destructive">{error}</p> : null}
    </div>
  );
}
