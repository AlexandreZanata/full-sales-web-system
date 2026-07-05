import { Package } from 'lucide-react';

import { ProductImageCarousel } from '@/components/catalog/ProductImageCarousel';
import { buildGallerySlides } from '@/lib/catalog/gallerySlides';
import type { PortalProductDetail } from '@/lib/api/types';
import { cn } from '@/lib/utils';

type ProductMediaPanelProps = {
  product: PortalProductDetail;
  className?: string;
};

export function ProductMediaPanel({ product, className }: ProductMediaPanelProps) {
  const slides = buildGallerySlides(product.name, product.primaryImageUrl, product.imageUrls);

  if (slides.length === 0) {
    const initial =
      product.name.trim().charAt(0).toUpperCase() || product.sku.charAt(0).toUpperCase();

    return (
      <div
        className={cn(
          'relative flex aspect-square items-center justify-center overflow-hidden rounded-lg border border-hairline bg-surface-muted text-muted-foreground lg:sticky lg:top-20',
          className,
        )}
        aria-hidden
      >
        <Package className="absolute size-16 opacity-20" strokeWidth={1.25} />
        <span className="relative text-3xl font-semibold text-foreground/70">{initial}</span>
      </div>
    );
  }

  return (
    <div className={cn('lg:sticky lg:top-20', className)}>
      <ProductImageCarousel slides={slides} />
    </div>
  );
}
