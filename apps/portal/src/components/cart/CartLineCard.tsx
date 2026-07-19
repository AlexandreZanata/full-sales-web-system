import { Trash2 } from 'lucide-react';

import type { CartLine } from '@/cart/CartProvider';
import { CartQuantityStepper } from '@/components/cart/CartQuantityStepper';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';

type CartLineCardProps = {
  line: CartLine;
  onSetQuantity: (productId: string, quantity: number) => void;
  onRemove: (productId: string) => void;
};

export function CartLineCard({ line, onSetQuantity, onRemove }: CartLineCardProps) {
  const { t } = useI18n();

  return (
    <li className="cart-line-card">
      <div className="cart-line-card__image">
        {line.primaryImageUrl ? (
          <img src={line.primaryImageUrl} alt="" className="size-full object-cover" />
        ) : null}
      </div>

      <div className="cart-line-card__body">
        <p className="cart-line-card__name">{line.name}</p>
        <p className="cart-line-card__sku">{line.sku}</p>
        <p className="cart-line-card__price">
          {formatMoney(line.unitPriceAmount * line.quantity, line.unitPriceCurrency)}
        </p>
        <CartQuantityStepper
          quantity={line.quantity}
          quantityLabel={t('common.quantity')}
          decreaseLabel={t('cart.decreaseQuantity')}
          increaseLabel={t('cart.increaseQuantity')}
          onDecrease={() => {
            onSetQuantity(line.productId, line.quantity - 1);
          }}
          onIncrease={() => {
            onSetQuantity(line.productId, line.quantity + 1);
          }}
        />
      </div>

      <button
        type="button"
        className="cart-line-card__actions"
        aria-label={t('cart.removeItem')}
        onClick={() => {
          onRemove(line.productId);
        }}
      >
        <span className="cart-line-card__remove-icon">
          <Trash2 className="size-4" aria-hidden />
        </span>
        <span className="cart-line-card__remove-label">{t('cart.removeItem')}</span>
      </button>
    </li>
  );
}
