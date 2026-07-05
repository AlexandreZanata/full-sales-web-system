import { Package } from 'lucide-react';

import type { PortalProduct } from '@/lib/api/types';
import { cn } from '@/lib/utils';

type ProductImageProps = {
  product: PortalProduct;
  className?: string;
  imageClassName?: string;
};

export function ProductImage({ product, className, imageClassName }: ProductImageProps) {
  if (product.primaryImageUrl) {
    return (
      <div className={cn('overflow-hidden bg-surface-muted', className)}>
        <img
          src={product.primaryImageUrl}
          alt={product.name}
          className={cn('size-full object-cover', imageClassName)}
          loading="lazy"
        />
      </div>
    );
  }

  const initial =
    product.name.trim().charAt(0).toUpperCase() || product.sku.charAt(0).toUpperCase();

  return (
    <div
      className={cn(
        'relative flex items-center justify-center overflow-hidden bg-surface-muted text-muted-foreground',
        className,
      )}
      aria-hidden
    >
      <Package className="absolute size-10 opacity-20" strokeWidth={1.25} />
      <span className="relative text-lg font-semibold text-foreground/70">{initial}</span>
    </div>
  );
}
