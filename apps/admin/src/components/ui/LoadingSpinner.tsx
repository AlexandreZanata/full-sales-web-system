import { cn } from '@/lib/utils';

type LoadingSpinnerProps = {
  className?: string;
  label?: string;
};

export function LoadingSpinner({ className, label = 'Loading' }: LoadingSpinnerProps) {
  return (
    <div
      className={cn(
        'flex items-center justify-center gap-2 text-sm text-muted-foreground',
        className,
      )}
    >
      <span
        className="size-4 animate-spin rounded-full border-2 border-hairline border-t-foreground"
        aria-hidden
      />
      <span>{label}</span>
    </div>
  );
}
