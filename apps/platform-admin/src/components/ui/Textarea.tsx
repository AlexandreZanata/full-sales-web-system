import { cn } from '@/lib/utils';
import { useId, type TextareaHTMLAttributes } from 'react';

type TextareaProps = TextareaHTMLAttributes<HTMLTextAreaElement> & {
  label?: string;
  error?: string;
};

export function Textarea({ className, label, error, id, ...props }: TextareaProps) {
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
      <textarea
        id={inputId}
        className={cn(
          'min-h-24 w-full rounded-lg border bg-surface px-3.5 py-2.5 text-sm text-foreground shadow-sm transition-shadow placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-admin-accent/40 focus:ring-offset-1',
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
