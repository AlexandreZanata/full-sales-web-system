import { createFileRoute } from '@tanstack/react-router';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useMemo, useState } from 'react';

import { EditProductForm } from '@/components/products/EditProductForm';
import { ProductImagesSection } from '@/components/products/ProductImagesSection';
import { StockBalanceCard } from '@/components/products/StockBalanceCard';
import { ActiveBadge } from '@/components/users/ActiveBadge';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import { LoadingSpinner } from '@/components/ui/LoadingSpinner';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { Tabs } from '@/components/ui/Tabs';
import { useToast } from '@/hooks/useToast';
import { fetchProduct, updateProduct } from '@/lib/api/products';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';

type ProductDetailSearch = {
  tab?: 'overview' | 'images';
};

export const Route = createFileRoute('/_authenticated/products/$id')({
  validateSearch: (search: Record<string, unknown>): ProductDetailSearch => ({
    tab: search.tab === 'images' ? 'images' : 'overview',
  }),
  component: ProductDetailPage,
});

function ProductDetailPage() {
  const { id } = Route.useParams();
  const { tab: initialTab } = Route.useSearch();
  const { t } = useI18n();
  const queryClient = useQueryClient();
  const toast = useToast();
  const [activeTab, setActiveTab] = useState(initialTab ?? 'overview');
  const [confirmOpen, setConfirmOpen] = useState(false);
  const [deactivating, setDeactivating] = useState(false);
  const [reactivating, setReactivating] = useState(false);

  const product = useQuery({
    queryKey: ['products', id],
    queryFn: () => fetchProduct(id),
  });

  const tabs = useMemo(
    () => [
      { id: 'overview', label: t('products.detail.tabs.overview') },
      { id: 'images', label: t('products.detail.tabs.images') },
    ],
    [t],
  );

  async function handleReactivate() {
    setReactivating(true);
    try {
      await updateProduct(id, { active: true });
      await queryClient.invalidateQueries({ queryKey: ['products'] });
      await queryClient.invalidateQueries({ queryKey: ['products', id] });
      toast.success(t('products.toast.reactivated'));
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setReactivating(false);
    }
  }

  async function handleDeactivate() {
    setDeactivating(true);
    try {
      await updateProduct(id, { active: false });
      await queryClient.invalidateQueries({ queryKey: ['products'] });
      await queryClient.invalidateQueries({ queryKey: ['products', id] });
      toast.success(t('products.toast.deactivated'));
      setConfirmOpen(false);
    } catch {
      toast.error(t('errors.actionFailed'));
    } finally {
      setDeactivating(false);
    }
  }

  if (product.isLoading) {
    return (
      <div className="flex justify-center py-16">
        <LoadingSpinner />
      </div>
    );
  }

  if (!product.data) {
    return (
      <PageHeader
        title={t('products.detail.notFound')}
        back={<PageBackLink label={t('common.backTo.products')} to="/products" />}
      />
    );
  }

  const detail = product.data;

  return (
    <div className="space-y-4">
      <PageHeader
        title={detail.name}
        description={detail.sku}
        back={<PageBackLink label={t('common.backTo.products')} to="/products" />}
        actions={
          detail.active ? (
            <Button
              variant="danger"
              onClick={() => {
                setConfirmOpen(true);
              }}
            >
              {t('products.detail.deactivate')}
            </Button>
          ) : (
            <Button disabled={reactivating} onClick={() => void handleReactivate()}>
              {t('products.actions.reactivate')}
            </Button>
          )
        }
      />

      <Tabs
        tabs={tabs}
        activeId={activeTab}
        onChange={(tabId) => {
          if (tabId === 'overview' || tabId === 'images') {
            setActiveTab(tabId);
          }
        }}
      >
        {activeTab === 'overview' ? (
          <div className="space-y-4">
            <Card className="space-y-3">
              <div className="flex flex-col gap-1 sm:flex-row sm:items-center sm:justify-between">
                <span className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
                  {t('forms.fields.price')}
                </span>
                <span className="text-sm text-foreground">
                  {formatMoney(detail.priceAmount, detail.priceCurrency)}
                </span>
              </div>
              <div className="flex flex-col gap-1 sm:flex-row sm:items-center sm:justify-between">
                <span className="text-xs font-semibold uppercase tracking-[0.12em] text-muted-foreground">
                  {t('forms.fields.status')}
                </span>
                <ActiveBadge active={detail.active} />
              </div>
            </Card>

            <StockBalanceCard productId={id} />

            <EditProductForm
              product={detail}
              onSubmit={(body) => updateProduct(id, body)}
              onUpdated={() => {
                void queryClient.invalidateQueries({ queryKey: ['products'] });
                void queryClient.invalidateQueries({ queryKey: ['products', id] });
              }}
            />
          </div>
        ) : null}

        {activeTab === 'images' ? <ProductImagesSection productId={id} /> : null}
      </Tabs>

      <ConfirmDialog
        open={confirmOpen}
        title={t('products.detail.deactivateDialog.title')}
        message={t('products.detail.deactivateDialog.message')}
        confirmLabel={t('products.detail.deactivateDialog.confirm')}
        destructive
        isLoading={deactivating}
        onCancel={() => {
          setConfirmOpen(false);
        }}
        onConfirm={() => void handleDeactivate()}
      />
    </div>
  );
}
