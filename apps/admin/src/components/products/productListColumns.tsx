import { Link } from '@tanstack/react-router';

import { ProductThumb } from '@/components/products/ProductThumb';
import { TableActions, tableActionClass } from '@/components/ui/TableActions';
import { ActiveBadge } from '@/components/users/ActiveBadge';
import type { DataTableColumn } from '@/components/ui/DataTable';
import type { ProductSummary } from '@/lib/api/types';
import type { MessageKey } from '@/lib/i18n/messages';
import { formatMoney } from '@/lib/products/formatPrice';

type Translate = (key: MessageKey) => string;

export function productListColumns(
  t: Translate,
  cacheRevision = 0,
): DataTableColumn<ProductSummary>[] {
  return [
    {
      id: 'image',
      header: t('common.table.image'),
      cell: (row) => (
        <ProductThumb
          name={row.name}
          primaryImageFileId={row.primaryImageFileId}
          primaryImageUrl={row.primaryImageUrl}
          cacheRevision={cacheRevision}
        />
      ),
    },
    {
      id: 'sku',
      header: t('forms.fields.sku'),
      cell: (row) => row.sku,
    },
    {
      id: 'name',
      header: t('common.table.name'),
      cell: (row) => (
        <Link to="/products/$id" params={{ id: row.id }} className="font-medium hover:underline">
          {row.name}
        </Link>
      ),
    },
    {
      id: 'category',
      header: t('forms.fields.category'),
      cell: (row) => row.categoryName ?? '—',
    },
    {
      id: 'price',
      header: t('forms.fields.price'),
      cell: (row) => formatMoney(row.priceAmount, row.priceCurrency),
    },
    {
      id: 'active',
      header: t('forms.fields.status'),
      cell: (row) => <ActiveBadge active={row.active} />,
    },
    {
      id: 'actions',
      header: t('common.table.actions'),
      align: 'right',
      cell: (row) => (
        <TableActions>
          <Link to="/products/$id" params={{ id: row.id }} className={tableActionClass('open')}>
            {t('common.edit')}
          </Link>
          <Link
            to="/products/$id"
            params={{ id: row.id }}
            search={{ tab: 'images' }}
            className={tableActionClass('primary')}
          >
            {t('products.actions.manageImages')}
          </Link>
        </TableActions>
      ),
    },
  ];
}
