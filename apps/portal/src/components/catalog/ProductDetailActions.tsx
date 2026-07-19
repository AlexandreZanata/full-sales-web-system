import { useNavigate } from '@tanstack/react-router';
import { MessageCircle, ShoppingBag, ShoppingCart } from 'lucide-react';

import { useCart } from '@/cart/CartProvider';
import { Button } from '@/components/ui/Button';
import type { PortalProductDetail } from '@/lib/api/types';
import { buildSellerWhatsAppLink } from '@/lib/contact/sellerWhatsAppLink';
import { useI18n } from '@/lib/i18n/context';
import { resolveContactPhone } from '@/lib/seller/attribution';
import { useSellerAttribution } from '@/lib/seller/useSellerAttribution';
import { cn } from '@/lib/utils';

type ProductDetailActionsProps = {
  product: PortalProductDetail;
  salesContactPhone?: string;
  productUrl: string;
  className?: string;
};

export function ProductDetailActions({
  product,
  salesContactPhone,
  productUrl,
  className,
}: ProductDetailActionsProps) {
  const { t } = useI18n();
  const navigate = useNavigate();
  const { addProduct } = useCart();
  const { attribution } = useSellerAttribution();
  const phone = resolveContactPhone(attribution, salesContactPhone);

  const contactHref = phone?.trim()
    ? buildSellerWhatsAppLink(phone, product, productUrl)
    : null;

  const placeOrder = () => {
    addProduct(product);
    void navigate({ to: '/cart' });
  };

  return (
    <div className={cn('product-detail-actions', className)}>
      <Button
        className="product-detail-actions__primary"
        onClick={() => {
          addProduct(product);
        }}
      >
        <ShoppingCart className="size-4" aria-hidden />
        {t('productDetail.addToCart')}
      </Button>

      <div className="product-detail-actions__secondary">
        <Button
          className="product-detail-actions__secondary-btn"
          variant="secondary"
          onClick={placeOrder}
        >
          <ShoppingBag className="size-4" aria-hidden />
          {t('productDetail.placeOrder')}
        </Button>
        {contactHref ? (
          <a
            href={contactHref}
            target="_blank"
            rel="noopener noreferrer"
            className="product-detail-actions__link"
          >
            <MessageCircle className="size-4" aria-hidden />
            {t('productDetail.contactSeller')}
          </a>
        ) : (
          <Button
            className="product-detail-actions__secondary-btn"
            variant="secondary"
            disabled
            title={t('productDetail.contactUnavailable')}
          >
            <MessageCircle className="size-4" aria-hidden />
            {t('productDetail.contactSeller')}
          </Button>
        )}
      </div>
    </div>
  );
}
