import { Link } from '@tanstack/react-router';
import { ShoppingCart } from 'lucide-react';

import { useCart } from '@/cart/CartProvider';
import { useI18n } from '@/lib/i18n/context';
import { cn } from '@/lib/utils';

type CartFabProps = {
  className?: string;
};

export function CartFab({ className }: CartFabProps) {
  const { itemCount } = useCart();
  const { t } = useI18n();

  if (itemCount === 0) {
    return null;
  }

  return (
    <Link
      to="/cart"
      className={cn(
        'fixed bottom-20 right-4 z-30 inline-flex size-14 items-center justify-center rounded-full bg-primary text-primary-foreground shadow-lg lg:hidden',
        className,
      )}
      aria-label={t('nav.cart')}
    >
      <ShoppingCart className="size-6" aria-hidden />
      <span className="absolute -right-1 -top-1 flex size-5 items-center justify-center rounded-full bg-accent text-[11px] font-semibold">
        {itemCount}
      </span>
    </Link>
  );
}
