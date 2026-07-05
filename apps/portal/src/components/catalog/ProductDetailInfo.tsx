import type { PortalProductDetail } from '@/lib/api/types';
import { useI18n } from '@/lib/i18n/context';
import { formatMoney } from '@/lib/products/formatPrice';

type ProductDetailInfoProps = {
  product: PortalProductDetail;
};

export function ProductDetailInfo({ product }: ProductDetailInfoProps) {
  const { t } = useI18n();

  return (
    <div className="space-y-6">
      <div className="space-y-3">
        <span className="inline-flex rounded-full border border-hairline bg-surface-muted px-2.5 py-0.5 text-xs font-medium text-muted-foreground">
          {t('catalog.sku')}: {product.sku}
        </span>
        <h1 className="text-2xl font-semibold text-foreground md:text-3xl">{product.name}</h1>
        {product.categoryName ? (
          <p className="text-sm text-muted-foreground">{product.categoryName}</p>
        ) : null}
        <p className="catalog-price catalog-price--prominent text-2xl">
          {formatMoney(product.priceAmount, product.priceCurrency)}
        </p>
      </div>

      {product.description ? (
        <section className="space-y-2">
          <h2 className="text-sm font-semibold text-foreground">
            {t('productDetail.description')}
          </h2>
          <p className="text-sm leading-relaxed text-muted-foreground">{product.description}</p>
        </section>
      ) : null}

      <section className="rounded-lg border border-hairline bg-surface p-4">
        <h2 className="mb-3 text-sm font-semibold text-foreground">
          {t('productDetail.specsTitle')}
        </h2>
        <dl className="grid gap-2 text-sm">
          <div className="flex justify-between gap-4">
            <dt className="text-muted-foreground">{t('productDetail.unitOfMeasure')}</dt>
            <dd className="font-medium text-foreground">{product.unitOfMeasure}</dd>
          </div>
          <div className="flex justify-between gap-4">
            <dt className="text-muted-foreground">{t('catalog.sku')}</dt>
            <dd className="font-medium text-foreground">{product.sku}</dd>
          </div>
          {product.categoryName ? (
            <div className="flex justify-between gap-4">
              <dt className="text-muted-foreground">{t('productDetail.category')}</dt>
              <dd className="font-medium text-foreground">{product.categoryName}</dd>
            </div>
          ) : null}
          <div className="flex justify-between gap-4">
            <dt className="text-muted-foreground">{t('productDetail.status')}</dt>
            <dd className="font-medium text-status-active">{t('productDetail.statusActive')}</dd>
          </div>
        </dl>
      </section>
    </div>
  );
}
