import { useMutation, useQuery } from '@tanstack/react-query';
import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { useMemo, useState } from 'react';

import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { Select } from '@/components/ui/Select';
import { fetchCommerces } from '@/lib/api/commerces';
import { fetchProducts, fetchStockBalance } from '@/lib/api/products';
import { createSale } from '@/lib/api/sales';
import { ApiError } from '@/lib/api/client';
import { useI18n } from '@/lib/i18n/context';
import { PAYMENT_METHODS, saleActionErrorMessage } from '@/lib/sales/constants';
import { formatMoney } from '@/lib/products/formatPrice';
import { randomId } from '@/lib/utils';

type Line = { productId: string; quantity: string };

export const Route = createFileRoute('/_authenticated/sales/new')({
  component: NewSalePage,
});

function NewSalePage() {
  const { t, tf, paymentMethod: paymentMethodLabel } = useI18n();
  const navigate = useNavigate();
  const [commerceSearch, setCommerceSearch] = useState('');
  const [commerceId, setCommerceId] = useState('');
  const [paymentMethod, setPaymentMethod] = useState('');
  const [lines, setLines] = useState<Line[]>([{ productId: '', quantity: '1' }]);
  const [error, setError] = useState<string | null>(null);

  const commercesQuery = useQuery({
    queryKey: ['commerces', commerceSearch],
    queryFn: () => fetchCommerces(commerceSearch),
  });

  const productsQuery = useQuery({
    queryKey: ['products'],
    queryFn: fetchProducts,
  });

  const selectedProductId = lines.find((line) => line.productId)?.productId ?? '';
  const stockQuery = useQuery({
    queryKey: ['stock', selectedProductId],
    queryFn: () => fetchStockBalance(selectedProductId),
    enabled: Boolean(selectedProductId),
  });

  const totalAmount = useMemo(() => {
    const products = productsQuery.data ?? [];
    return lines.reduce((sum, line) => {
      const product = products.find((item) => item.id === line.productId);
      const qty = Number(line.quantity);
      if (!product || !Number.isFinite(qty) || qty <= 0) return sum;
      return sum + product.priceAmount * qty;
    }, 0);
  }, [lines, productsQuery.data]);

  const createMutation = useMutation({
    mutationFn: () =>
      createSale(
        {
          commerceId,
          paymentMethod,
          items: lines
            .filter((line) => line.productId && Number(line.quantity) > 0)
            .map((line) => ({ productId: line.productId, quantity: Number(line.quantity) })),
        },
        randomId(),
      ),
    onSuccess: (sale) => {
      void navigate({ to: '/sales/$id', params: { id: sale.id } });
    },
    onError: (err) => {
      setError(err instanceof ApiError ? saleActionErrorMessage(err.code) : t('common.loadFailed'));
    },
  });

  if (commercesQuery.isLoading || productsQuery.isLoading) {
    return <LoadingSpinner className="py-16" />;
  }

  return (
    <div className="space-y-4 pb-24">
      <h1 className="text-2xl font-semibold">{t('sales.new')}</h1>
      <Card className="space-y-4">
        <Input
          label={t('common.search')}
          value={commerceSearch}
          onChange={(e) => {
            setCommerceSearch(e.target.value);
          }}
          placeholder={t('sales.selectCommerce')}
        />
        <Select
          label={t('sales.commerce')}
          value={commerceId}
          onChange={(e) => {
            setCommerceId(e.target.value);
          }}
        >
          <option value="">{t('sales.selectCommerce')}</option>
          {(commercesQuery.data ?? []).map((commerce) => (
            <option key={commerce.id} value={commerce.id}>
              {commerce.tradeName || commerce.legalName}
            </option>
          ))}
        </Select>

        <Select
          label={t('sales.paymentMethod')}
          value={paymentMethod}
          onChange={(e) => {
            setPaymentMethod(e.target.value);
          }}
        >
          <option value="">{t('sales.paymentMethod')}</option>
          {PAYMENT_METHODS.map((method) => (
            <option key={method} value={method}>
              {paymentMethodLabel(method)}
            </option>
          ))}
        </Select>

        {lines.map((line, index) => (
          <div
            key={index}
            className="grid gap-2 rounded-md border border-hairline p-3 sm:grid-cols-2"
          >
            <Select
              label={t('sales.product')}
              value={line.productId}
              onChange={(e) => {
                const next = [...lines];
                next[index] = { ...line, productId: e.target.value };
                setLines(next);
              }}
            >
              <option value="">{t('sales.selectProduct')}</option>
              {(productsQuery.data ?? []).map((product) => (
                <option key={product.id} value={product.id}>
                  {product.name} ({product.sku})
                </option>
              ))}
            </Select>
            <Input
              label={t('common.quantity')}
              type="number"
              min={1}
              value={line.quantity}
              onChange={(e) => {
                const next = [...lines];
                next[index] = { ...line, quantity: e.target.value };
                setLines(next);
              }}
            />
            {line.productId === selectedProductId && stockQuery.data ? (
              <p className="text-xs text-muted-foreground sm:col-span-2">
                {tf('common.stockAvailable', { qty: stockQuery.data.available })}
              </p>
            ) : null}
          </div>
        ))}

        <Button
          variant="secondary"
          onClick={() => {
            setLines((current) => [...current, { productId: '', quantity: '1' }]);
          }}
        >
          {t('sales.addLine')}
        </Button>

        <div className="flex justify-between border-t border-hairline pt-3 text-sm font-semibold">
          <span>{t('common.total')}</span>
          <span>{formatMoney(totalAmount)}</span>
        </div>

        {error ? <p className="text-sm text-destructive">{error}</p> : null}
      </Card>

      <div className="fixed inset-x-0 bottom-16 border-t border-hairline bg-surface p-4 md:static md:border-0 md:bg-transparent md:p-0">
        <Button
          className="w-full md:w-auto"
          disabled={createMutation.isPending || !commerceId || !paymentMethod}
          onClick={() => {
            setError(null);
            createMutation.mutate();
          }}
        >
          {createMutation.isPending ? t('common.working') : t('common.confirm')}
        </Button>
      </div>
    </div>
  );
}
