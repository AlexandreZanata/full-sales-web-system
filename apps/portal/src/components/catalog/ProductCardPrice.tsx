import { formatMoney } from '@/lib/products/formatPrice';

type ProductCardPriceProps = {
  priceAmount: number;
  priceCurrency: string;
  compareAtPrice?: number;
};

export function ProductCardPrice({
  priceAmount,
  priceCurrency,
  compareAtPrice,
}: ProductCardPriceProps) {
  const formatted = formatMoney(priceAmount, priceCurrency);

  if (compareAtPrice && compareAtPrice > priceAmount) {
    return (
      <div className="flex flex-wrap items-baseline gap-2">
        <span className="catalog-price catalog-price--prominent">{formatted}</span>
        <span className="text-sm font-medium text-muted-foreground line-through tabular-nums">
          {formatMoney(compareAtPrice, priceCurrency)}
        </span>
      </div>
    );
  }

  return <p className="catalog-price catalog-price--prominent">{formatted}</p>;
}
