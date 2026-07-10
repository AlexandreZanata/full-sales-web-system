import { cn } from '@/lib/utils';

const BRAND_MARK_SIZES = {
  sm: 'h-9 w-9',
  md: 'h-11 w-11',
  lg: 'h-14 w-14',
} as const;

type BrandMarkSize = keyof typeof BRAND_MARK_SIZES;

type BrandMarkProps = {
  size?: BrandMarkSize;
  className?: string;
};

export function BrandMark({ size = 'md', className }: BrandMarkProps) {
  return (
    <div
      aria-hidden
      className={cn(
        'flex shrink-0 items-center justify-center rounded-md bg-primary text-xs font-bold text-primary-foreground',
        BRAND_MARK_SIZES[size],
        className,
      )}
    >
      FS
    </div>
  );
}
