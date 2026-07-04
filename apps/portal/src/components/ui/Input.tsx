import { cn } from '@/lib/utils';
import { useId, type InputHTMLAttributes } from 'react';

type InputProps = InputHTMLAttributes<HTMLInputElement> & {
  label?: string;
  error?: string;
};

export function Input({ className, label, error, id, ...props }: InputProps) {
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
      <input
        id={inputId}
        className={cn(
          'h-10 w-full rounded-md border bg-surface px-3 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-accent',
          error ? 'border-destructive focus:ring-destructive' : 'border-input',
          className,
        )}
        aria-invalid={Boolean(error)}
        {...props}
      />
      {error ? <p className="text-xs text-destructive">{error}</p> : null}
    </div>
  );
}
