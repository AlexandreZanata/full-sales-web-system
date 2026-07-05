/**
 * Contract: Phase 44 category reorder helper.
 */
import { describe, expect, it } from 'vitest';

import { canMoveCategory, moveCategoryInOrder } from '@/lib/categories/reorder';

describe('category reorder helper — Phase 44 contract', () => {
  const orderedIds = ['a', 'b', 'c'];

  it('given_first_item_when_move_up_then_order_unchanged', () => {
    expect(moveCategoryInOrder(orderedIds, 'a', 'up')).toEqual(['a', 'b', 'c']);
    expect(canMoveCategory(orderedIds, 'a', 'up')).toBe(false);
  });

  it('given_middle_item_when_move_down_then_swaps_with_next', () => {
    expect(moveCategoryInOrder(orderedIds, 'b', 'down')).toEqual(['a', 'c', 'b']);
    expect(canMoveCategory(orderedIds, 'b', 'down')).toBe(true);
  });

  it('given_last_item_when_move_down_then_order_unchanged', () => {
    expect(moveCategoryInOrder(orderedIds, 'c', 'down')).toEqual(['a', 'b', 'c']);
    expect(canMoveCategory(orderedIds, 'c', 'down')).toBe(false);
  });
});
