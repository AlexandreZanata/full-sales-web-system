import { useNavigate } from '@tanstack/react-router';

import { useCart } from '@/cart/CartProvider';
import { Button } from '@/components/ui/Button';
import type { PortalProductDetail } from '@/lib/api/types';
import { buildSellerWhatsAppLink } from '@/lib/contact/sellerWhatsAppLink';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

const actionLinkClass =
  'inline-flex h-10 min-h-10 items-center justify-center rounded-md border border-hairline bg-surface px-4 text-sm font-semibold text-foreground transition-colors hover:bg-surface-muted';

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

  const contactHref =
    salesContactPhone?.trim() ?
      buildSellerWhatsAppLink(salesContactPhone, product, productUrl)
    : null;

  const placeOrder = () => {
    addProduct(product);
    void navigate({ to: '/cart' });
  };

  return (
    <div className={cn('flex flex-col gap-2 sm:flex-row sm:flex-wrap', className)}>
      <Button
        className="w-full sm:w-auto sm:min-w-[10rem]"
        onClick={() => {
          addProduct(product);
        }}
      >
        {t('productDetail.addToCart')}
      </Button>
      <Button className="w-full sm:w-auto sm:min-w-[10rem]" variant="secondary" onClick={placeOrder}>
        {t('productDetail.placeOrder')}
      </Button>
      {contactHref ? (
        <a
          href={contactHref}
          target="_blank"
          rel="noopener noreferrer"
          className={cn(actionLinkClass, 'w-full sm:w-auto sm:min-w-[10rem]')}
        >
          {t('productDetail.contactSeller')}
        </a>
      ) : (
        <Button
          className="w-full sm:w-auto sm:min-w-[10rem]"
          variant="secondary"
          disabled
          title={t('productDetail.contactUnavailable')}
        >
          {t('productDetail.contactSeller')}
        </Button>
      )}
    </div>
  );
}
