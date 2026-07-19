import { useId, type InputHTMLAttributes, type ReactNode } from 'react';

import { cn } from '@/lib/utils';

type LoginTextFieldProps = InputHTMLAttributes<HTMLInputElement> & {
  label: string;
  leftIcon?: ReactNode;
  rightSlot?: ReactNode;
};

export function LoginTextField({
  label,
  leftIcon,
  rightSlot,
  className,
  id,
  ...props
}: LoginTextFieldProps) {
  const generatedId = useId();
  const inputId = id ?? props.name ?? generatedId;

  return (
    <div className="space-y-1.5">
      <label
        htmlFor={inputId}
        className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground"
      >
        {label}
      </label>
      <div className="relative">
        {leftIcon ? (
          <span className="pointer-events-none absolute inset-y-0 left-3 flex items-center text-muted-foreground">
            {leftIcon}
          </span>
        ) : null}
        <input
          id={inputId}
          className={cn(
            'h-11 w-full rounded-lg border border-input bg-surface text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-admin-login-accent',
            leftIcon ? 'pl-10' : 'pl-3',
            rightSlot ? 'pr-10' : 'pr-3',
            className,
          )}
          {...props}
        />
        {rightSlot ? (
          <span className="absolute inset-y-0 right-2 flex items-center">{rightSlot}</span>
        ) : null}
      </div>
    </div>
  );
}
