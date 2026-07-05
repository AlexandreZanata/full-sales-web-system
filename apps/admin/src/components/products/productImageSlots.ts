import type { ProductImage } from '@/lib/api/types';

export const MAX_PRODUCT_IMAGES = 5;

export function slotCount(images: ProductImage[]): number {
  if (images.length >= MAX_PRODUCT_IMAGES) {
    return MAX_PRODUCT_IMAGES;
  }
  const maxSort = images.reduce((max, image) => Math.max(max, image.sortOrder), -1);
  return Math.min(MAX_PRODUCT_IMAGES, Math.max(maxSort + 2, 1));
}

export function imagesBySlot(images: ProductImage[], slots: number): Array<ProductImage | undefined> {
  const bySlot: Array<ProductImage | undefined> = Array.from({ length: slots }, () => undefined);
  const unplaced: ProductImage[] = [];

  for (const image of images) {
    if (image.sortOrder >= 0 && image.sortOrder < slots && !bySlot[image.sortOrder]) {
      bySlot[image.sortOrder] = image;
      continue;
    }
    unplaced.push(image);
  }

  for (const image of unplaced) {
    const emptyIndex = bySlot.findIndex((slot) => slot === undefined);
    if (emptyIndex === -1) {
      break;
    }
    bySlot[emptyIndex] = image;
  }

  return bySlot;
}
