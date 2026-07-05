import { ChevronLeft, ChevronRight } from 'lucide-react';
import { useCallback, useState } from 'react';

import type { GallerySlide } from '@/lib/catalog/gallerySlides';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type ProductImageCarouselProps = {
  slides: GallerySlide[];
  className?: string;
};

export function ProductImageCarousel({ slides, className }: ProductImageCarouselProps) {
  const { t } = useI18n();
  const [index, setIndex] = useState(0);

  const goTo = useCallback(
    (next: number) => {
      if (slides.length === 0) {
        return;
      }
      setIndex((next + slides.length) % slides.length);
    },
    [slides.length],
  );

  if (slides.length === 0) {
    return null;
  }

  const safeIndex = ((index % slides.length) + slides.length) % slides.length;
  const slide = slides[safeIndex];

  if (slides.length === 1) {
    return (
      <div
        className={cn(
          'overflow-hidden rounded-lg border border-hairline bg-surface-muted',
          className,
        )}
      >
        <img src={slide.url} alt={slide.alt} className="aspect-square w-full object-cover" />
      </div>
    );
  }

  return (
    <div
      className={cn(
        'relative overflow-hidden rounded-lg border border-hairline bg-surface-muted',
        className,
      )}
      aria-roledescription="carousel"
      aria-label={t('productDetail.imageCarousel')}
    >
      <img src={slide.url} alt={slide.alt} className="aspect-square w-full object-cover" />
      <button
        type="button"
        className="absolute left-2 top-1/2 flex size-9 -translate-y-1/2 items-center justify-center rounded-full border border-hairline bg-surface/90 text-foreground shadow-sm"
        aria-label={t('productDetail.prevImage')}
        onClick={() => {
          goTo(index - 1);
        }}
      >
        <ChevronLeft className="size-5" aria-hidden />
      </button>
      <button
        type="button"
        className="absolute right-2 top-1/2 flex size-9 -translate-y-1/2 items-center justify-center rounded-full border border-hairline bg-surface/90 text-foreground shadow-sm"
        aria-label={t('productDetail.nextImage')}
        onClick={() => {
          goTo(index + 1);
        }}
      >
        <ChevronRight className="size-5" aria-hidden />
      </button>
      <div className="absolute inset-x-0 bottom-3 flex justify-center gap-1.5">
        {slides.map((item, dotIndex) => (
          <button
            key={item.url}
            type="button"
            className={cn(
              'size-2 rounded-full transition-colors',
              dotIndex === safeIndex ? 'bg-primary' : 'bg-surface/80',
            )}
            aria-label={`${t('productDetail.goToSlide')} ${String(dotIndex + 1)}`}
            aria-current={dotIndex === safeIndex ? 'true' : undefined}
            onClick={() => {
              setIndex(dotIndex);
            }}
          />
        ))}
      </div>
    </div>
  );
}
