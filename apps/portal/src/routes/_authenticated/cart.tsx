import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Link, createFileRoute, useNavigate } from '@tanstack/react-router';
import { useState } from 'react';

import { usePortalAuth } from '@/auth/usePortalAuth';
import { useCart } from '@/cart/CartProvider';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { EmptyState } from '@/components/ui/EmptyState';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { createPortalOrder, submitPortalOrder } from '@/lib/api/portal';
import { ApiError } from '@/lib/api/client';
import { setPostLoginRedirect } from '@/lib/auth/postLoginRedirect';
import { useI18n } from '@/lib/i18n/context';
import { addressesForCommerce } from '@/lib/orders/constants';
import { formatMoney } from '@/lib/products/formatPrice';

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
        <h1 className="text-2xl font-semibold text-foreground">{t('cart.title')}</h1>
        <ul className="space-y-3">
          {lines.map((line) => (
            <li key={line.productId} className="rounded-lg border border-hairline bg-surface p-4">
              <div className="flex gap-3">
                <div className="size-16 shrink-0 overflow-hidden rounded-md bg-surface-muted">
                  {line.primaryImageUrl ? (
                    <img src={line.primaryImageUrl} alt="" className="size-full object-cover" />
                  ) : null}
                </div>
                <div className="min-w-0 flex-1">
                  <p className="font-medium text-foreground">{line.name}</p>
                  <p className="text-xs text-muted-foreground">{line.sku}</p>
                  <p className="mt-1 text-sm font-semibold">
                    {formatMoney(line.unitPriceAmount * line.quantity, line.unitPriceCurrency)}
                  </p>
                  <div className="mt-2 flex items-center gap-2">
                    <Input
                      type="number"
                      min={1}
                      value={line.quantity}
                      aria-label={t('common.quantity')}
                      className="w-20"
                      onChange={(event) => {
                        setQuantity(line.productId, Number(event.target.value));
                      }}
                    />
                    <Button
                      variant="ghost"
                      onClick={() => {
                        removeLine(line.productId);
                      }}
                    >
                      {t('cart.removeItem')}
                    </Button>
                  </div>
                </div>
              </div>
            </li>
          ))}
        </ul>
      </div>

      <Card className="h-fit space-y-4 lg:sticky lg:top-20">
        <h2 className="text-lg font-semibold">{t('cart.checkout')}</h2>
        <Select
          label={t('cart.deliveryAddress')}
          value={deliveryAddressId}
          onChange={(event) => {
            setDeliveryAddressId(event.target.value);
          }}
        >
          {addresses.map((address) => (
            <option key={address.id} value={address.id}>
              {address.label}
            </option>
          ))}
        </Select>
        <Input
          label={t('cart.notes')}
          placeholder={t('cart.notesPlaceholder')}
          value={notes}
          onChange={(event) => {
            setNotes(event.target.value);
          }}
        />
        <div className="flex items-center justify-between border-t border-hairline pt-3 text-sm">
          <span className="text-muted-foreground">{t('common.total')}</span>
          <span className="text-lg font-semibold">{formatMoney(totalAmount, currency)}</span>
        </div>
        {error ? <p className="text-sm text-destructive">{error}</p> : null}
        {!user ? (
          <p className="text-sm text-muted-foreground">{t('cart.loginRequiredMessage')}</p>
        ) : null}
        <Button
          className="w-full"
          disabled={checkoutMutation.isPending || !deliveryAddressId}
          onClick={handleSubmitOrder}
        >
          {checkoutMutation.isPending
            ? t('common.working')
            : user
              ? t('cart.submitOrder')
              : t('cart.loginToCheckout')}
        </Button>
      </Card>
    </div>
  );
}
