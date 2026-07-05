import { describe, expect, it } from 'vitest';

import { imagesBySlot, slotCount } from '@/components/products/productImageSlots';
import type { ProductImage } from '@/lib/api/types';

function image(id: string, sortOrder: number, isPrimary = false): ProductImage {
  return {
    id,
    fileId: `file-${id}`,
    sortOrder,
    isPrimary,
  };
}

describe('productImageSlots', () => {
  it('slotCount keeps one empty slot after the highest sort order', () => {
    expect(slotCount([])).toBe(1);
    expect(slotCount([image('a', 0), image('b', 2)])).toBe(4);
  });

  it('imagesBySlot maps images to their sortOrder index', () => {
    const slots = imagesBySlot([image('a', 0), image('b', 2)], 3);

    expect(slots[0]?.id).toBe('a');
    expect(slots[1]).toBeUndefined();
    expect(slots[2]?.id).toBe('b');
  });

  it('imagesBySlot places legacy duplicate sort orders in the next free slot', () => {
    const slots = imagesBySlot([image('a', 0), image('b', 0)], 2);

    expect(slots[0]?.id).toBe('a');
    expect(slots[1]?.id).toBe('b');
  });

  it('slotCount caps at five images without an extra empty slot', () => {
    const five = Array.from({ length: 5 }, (_, index) => image(`img-${index}`, index));
    expect(slotCount(five)).toBe(5);
    expect(slotCount([...five, image('extra', 5)])).toBe(5);
  });
});
