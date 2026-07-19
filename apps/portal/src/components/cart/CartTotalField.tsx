import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';

type CartTotalFieldProps = {
  totalAmount: number;
  currency: string;
};

export function CartTotalField({ totalAmount, currency }: CartTotalFieldProps) {
  const { t } = useI18n();

  return (
    <div className="cart-total-field" data-testid="cart-total-value">
      <span className="cart-total-field__label">{t('cart.totalValue')}</span>
      <span className="cart-total-field__amount">{formatMoney(totalAmount, currency)}</span>
    </div>
  );
}
