import { cn } from '@/lib/utils';
import { type HTMLAttributes } from 'react';

export function Card({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return (
    <div className={cn('rounded-lg border border-hairline bg-surface p-6', className)} {...props} />
  );
}
