/** Strips HTML tags for plain-text card descriptions. */
export function stripHtml(html: string): string {
  return html
    .replace(/<[^>]*>/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

export function productCardDescription(description?: string): string | undefined {
  if (!description?.trim()) {
    return undefined;
  }
  const plain = stripHtml(description);
  return plain || undefined;
}
