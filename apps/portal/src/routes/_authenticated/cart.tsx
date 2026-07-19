import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Link, createFileRoute, useNavigate } from '@tanstack/react-router';
import { useMemo, useState } from 'react';

import { usePortalAuth } from '@/auth/usePortalAuth';
import { useCart } from '@/cart/CartProvider';
import { CartCheckoutPanel } from '@/components/cart/CartCheckoutPanel';
import { CartLineCard } from '@/components/cart/CartLineCard';
import { CartSecureNotice } from '@/components/cart/CartSecureNotice';
import { CartTotalField } from '@/components/cart/CartTotalField';
import { Button } from '@/components/ui/Button';
import { EmptyState } from '@/components/ui/EmptyState';
import { createPortalOrder, submitPortalOrder } from '@/lib/api/portal';
import { ApiError } from '@/lib/api/client';
import { setPostLoginRedirect } from '@/lib/auth/postLoginRedirect';
import { formatCartWhatsAppMessage } from '@/lib/contact/cartWhatsAppMessage';
import { buildSellerWhatsAppLink } from '@/lib/contact/sellerWhatsAppLink';
import { useI18n } from '@/lib/i18n/context';
import { addressesForCommerce } from '@/lib/orders/constants';
import { resolveContactPhone } from '@/lib/seller/attribution';
import { useSellerAttribution } from '@/lib/seller/useSellerAttribution';
import { useSiteSettings } from '@/lib/settings/useSiteSettings';

export const Route = createFileRoute('/_authenticated/cart')({
  component: CartPage,
});

function CartPage() {
  const { t } = useI18n();
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { user } = usePortalAuth();
  const { lines, itemCount, totalAmount, currency, setQuantity, removeLine, clear } = useCart();
  const addresses = addressesForCommerce(user?.commerceId);
  const [deliveryAddressId, setDeliveryAddressId] = useState(addresses[0]?.id ?? '');
  const [notes, setNotes] = useState('');
  const [error, setError] = useState<string | null>(null);
  const { attribution } = useSellerAttribution();
  const settingsQuery = useSiteSettings(true);
  const phone = resolveContactPhone(attribution, settingsQuery.data?.salesContactPhone);

  const contactHref = useMemo(() => {
    if (!phone?.trim() || lines.length === 0) {
      return null;
    }
    const message = formatCartWhatsAppMessage(
      t('cart.whatsappIntro'),
      t('cart.whatsappItem'),
      lines,
    );
    try {
      return buildSellerWhatsAppLink(phone, message);
    } catch {
      return null;
    }
  }, [phone, lines, t]);

  const checkoutMutation = useMutation({
    mutationFn: async () => {
      const order = await createPortalOrder({
        deliveryAddressId,
        notes: notes.trim() || undefined,
        items: lines.map((line) => ({ productId: line.productId, quantity: line.quantity })),
      });
      return submitPortalOrder(order.id);
    },
    onSuccess: async (order) => {
      clear();
      await queryClient.invalidateQueries({ queryKey: ['portal', 'orders'] });
      void navigate({ to: '/orders/$id', params: { id: order.id } });
    },
    onError: (err) => {
      setError(err instanceof ApiError ? err.message : t('common.error.loadFailed'));
    },
  });

  function handleSubmitOrder() {
    if (!user) {
      setPostLoginRedirect('/cart');
      void navigate({ to: '/login', search: { redirect: '/cart' } });
      return;
    }
    setError(null);
    checkoutMutation.mutate();
  }

  if (itemCount === 0) {
    return (
      <EmptyState
        title={t('common.empty.cart')}
        description={t('cart.emptyDescription')}
        action={
          <Link to="/">
            <Button>{t('common.backToCatalog')}</Button>
          </Link>
        }
      />
    );
  }

  return (
    <div className="grid gap-6 lg:grid-cols-[1fr_320px]">
      <div className="space-y-4">
        <h1 className="sr-only lg:not-sr-only lg:text-2xl lg:font-semibold lg:text-foreground">
          {t('cart.title')}
        </h1>
        <ul className="space-y-3">
          {lines.map((line) => (
            <CartLineCard
              key={line.productId}
              line={line}
              onSetQuantity={setQuantity}
              onRemove={removeLine}
            />
          ))}
        </ul>
        <CartSecureNotice />
        <CartTotalField totalAmount={totalAmount} currency={currency} />
      </div>

      <CartCheckoutPanel
        addresses={addresses}
        deliveryAddressId={deliveryAddressId}
        onDeliveryAddressChange={setDeliveryAddressId}
        notes={notes}
        onNotesChange={setNotes}
        totalAmount={totalAmount}
        currency={currency}
        error={error}
        loginHint={
          !user ? (
            <p className="text-sm text-muted-foreground">{t('cart.loginRequiredMessage')}</p>
          ) : null
        }
        contactHref={contactHref}
        submitLabel={
          checkoutMutation.isPending
            ? t('common.working')
            : user
              ? t('cart.submitOrder')
              : t('cart.loginToCheckout')
        }
        submitDisabled={checkoutMutation.isPending || !deliveryAddressId}
        onSubmit={handleSubmitOrder}
      />
    </div>
  );
}
