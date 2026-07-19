import { MessageCircle } from 'lucide-react';
import type { ReactNode } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { Select } from '@/components/ui/Select';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';

type AddressOption = { id: string; label: string };

type CartCheckoutPanelProps = {
  addresses: AddressOption[];
  deliveryAddressId: string;
  onDeliveryAddressChange: (id: string) => void;
  notes: string;
  onNotesChange: (value: string) => void;
  totalAmount: number;
  currency: string;
  error: string | null;
  loginHint: ReactNode;
  contactHref: string | null;
  submitLabel: string;
  submitDisabled: boolean;
  onSubmit: () => void;
};

export function CartCheckoutPanel({
  addresses,
  deliveryAddressId,
  onDeliveryAddressChange,
  notes,
  onNotesChange,
  totalAmount,
  currency,
  error,
  loginHint,
  contactHref,
  submitLabel,
  submitDisabled,
  onSubmit,
}: CartCheckoutPanelProps) {
  const { t } = useI18n();

  return (
    <Card className="h-fit space-y-4 lg:sticky lg:top-20">
      <h2 className="text-lg font-semibold">{t('cart.checkout')}</h2>
      <div className="cart-checkout-total">
        <span className="cart-checkout-total__label">{t('cart.totalValue')}</span>
        <span className="cart-checkout-total__amount" data-testid="cart-checkout-total">
          {formatMoney(totalAmount, currency)}
        </span>
      </div>
      <Select
        label={t('cart.deliveryAddress')}
        value={deliveryAddressId}
        onChange={(event) => {
          onDeliveryAddressChange(event.target.value);
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
          onNotesChange(event.target.value);
        }}
      />
      {error ? <p className="text-sm text-destructive">{error}</p> : null}
      {loginHint}
      {contactHref ? (
        <a
          href={contactHref}
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex w-full items-center justify-center gap-2 rounded-md bg-emerald-600 px-4 py-2.5 text-sm font-semibold text-white hover:bg-emerald-700"
          data-testid="cart-contact-seller"
        >
          <MessageCircle className="size-4" aria-hidden />
          {t('cart.contactSeller')}
        </a>
      ) : (
        <Button
          className="w-full"
          variant="secondary"
          disabled
          title={t('cart.contactUnavailable')}
        >
          <MessageCircle className="size-4" aria-hidden />
          {t('cart.contactSeller')}
        </Button>
      )}
      <Button className="w-full" disabled={submitDisabled} onClick={onSubmit}>
        {submitLabel}
      </Button>
    </Card>
  );
}
