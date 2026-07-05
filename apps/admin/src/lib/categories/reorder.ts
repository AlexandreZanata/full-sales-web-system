export function moveCategoryInOrder(
  orderedIds: readonly string[],
  categoryId: string,
  direction: 'up' | 'down',
): string[] {
  const index = orderedIds.indexOf(categoryId);
  if (index === -1) {
    return [...orderedIds];
  }

  const targetIndex = direction === 'up' ? index - 1 : index + 1;
  if (targetIndex < 0 || targetIndex >= orderedIds.length) {
    return [...orderedIds];
  }

  const next = [...orderedIds];
  [next[index], next[targetIndex]] = [next[targetIndex], next[index]];
  return next;
}

export function canMoveCategory(
  orderedIds: readonly string[],
  categoryId: string,
  direction: 'up' | 'down',
): boolean {
  const index = orderedIds.indexOf(categoryId);
  if (index === -1) {
    return false;
  }
  return direction === 'up' ? index > 0 : index < orderedIds.length - 1;
}
