import { Link } from '@tanstack/react-router';
import { useMemo } from 'react';

import type { DataTableColumn } from '@/components/ui/DataTable';
import { formatMoney } from '@/lib/products/formatPrice';
import { productDisplayName } from '@/lib/products/productNameMap';

export type ProductLineRow = {
  productId: string;
  quantity: number;
  unitPriceAmount: number;
  unitPriceCurrency: string;
  lineTotalAmount: number;
};

type Labels = {
  product: string;
  qty: string;
  unitPrice: string;
  lineTotal: string;
};

export function useProductLineColumns(
  labels: Labels,
  productNames: Map<string, string>,
): DataTableColumn<ProductLineRow>[] {
  return useMemo(
    () => [
      {
        id: 'product',
        header: labels.product,
        cell: (row) => (
          <Link
            to="/products/$id"
            params={{ id: row.productId }}
            className="text-sm hover:underline"
            onClick={(event) => {
              event.stopPropagation();
            }}
          >
            {productDisplayName(productNames, row.productId)}
          </Link>
        ),
      },
      {
        id: 'quantity',
        header: labels.qty,
        align: 'right',
        cell: (row) => row.quantity,
      },
      {
        id: 'unitPrice',
        header: labels.unitPrice,
        align: 'right',
        cell: (row) => formatMoney(row.unitPriceAmount, row.unitPriceCurrency),
      },
      {
        id: 'lineTotal',
        header: labels.lineTotal,
        align: 'right',
        cell: (row) => formatMoney(row.lineTotalAmount, row.unitPriceCurrency),
      },
    ],
    [labels.product, labels.qty, labels.unitPrice, labels.lineTotal, productNames],
  );
}
