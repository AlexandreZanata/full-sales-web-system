import { Lock } from 'lucide-react';

import { useI18n } from '@/lib/i18n/context';

export function CartSecureNotice() {
  const { t } = useI18n();

  return (
    <p className="cart-secure-notice">
      <Lock className="cart-secure-notice__icon" aria-hidden />
      <span>{t('cart.secureNotice')}</span>
    </p>
  );
}
