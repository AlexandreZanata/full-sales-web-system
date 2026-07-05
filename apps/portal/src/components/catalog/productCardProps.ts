import type { PortalProduct } from '@/lib/api/types';

export type ProductCardProps = {
  product: PortalProduct;
  onAddToCart: (product: PortalProduct) => void;
  onOpenDetail?: (product: PortalProduct) => void;
  addToCartLabel: string;
  skuLabel: string;
};
