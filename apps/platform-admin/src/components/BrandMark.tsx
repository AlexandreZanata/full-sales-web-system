import { cn } from '@/lib/utils';

import { PLATFORM_BRAND_NAME } from '@/lib/brand';

const BRAND_MARK_SIZES = {
  sm: 'h-9 w-9 text-[0.65rem]',
  md: 'h-11 w-11 text-xs',
  lg: 'h-14 w-14 text-sm',
} as const;

type BrandMarkSize = keyof typeof BRAND_MARK_SIZES;

type BrandMarkProps = {
  size?: BrandMarkSize;
  className?: string;
  variant?: 'default' | 'login';
};

export function BrandMark({ size = 'md', className, variant = 'default' }: BrandMarkProps) {
  const initials = PLATFORM_BRAND_NAME.slice(0, 2).toUpperCase();

  return (
    <div
      aria-hidden
      className={cn(
        'flex shrink-0 items-center justify-center rounded-lg font-bold tracking-tight',
        variant === 'login'
          ? 'bg-white/15 text-white ring-2 ring-white/30 backdrop-blur-sm'
          : 'bg-primary text-primary-foreground',
        BRAND_MARK_SIZES[size],
        className,
      )}
    >
      {initials}
    </div>
  );
}
