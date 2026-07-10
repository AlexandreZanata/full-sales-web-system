export function isPortalHomeActive(pathname: string, category?: string): boolean {
  return pathname === '/' && !category;
}

export function isPortalMenuActive(pathname: string, category?: string): boolean {
  if (category) {
    return true;
  }
  return pathname.startsWith('/products/');
}

export function isPortalOffersActive(hash: string): boolean {
  return hash === '#offers' || hash === '#offer-banners';
}
