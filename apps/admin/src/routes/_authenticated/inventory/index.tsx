import { Link, createFileRoute } from '@tanstack/react-router';
import { ClipboardList, SlidersHorizontal } from 'lucide-react';

import { Card } from '@/components/ui/Card';
import { PageHeader } from '@/components/ui/PageHeader';

export const Route = createFileRoute('/_authenticated/inventory/')({
  component: InventoryHubPage,
});

const links = [
  {
    to: '/inventory/adjustments',
    label: 'Adjustments',
    description: 'Record manual stock corrections with a reason.',
    icon: SlidersHorizontal,
  },
  {
    to: '/inventory/ledger',
    label: 'Ledger',
    description: 'Review paginated movement history by product.',
    icon: ClipboardList,
  },
] as const;

function InventoryHubPage() {
  return (
    <div>
      <PageHeader title="Inventory" description="Stock balances and movement audit trail." />

      <div className="grid gap-4 sm:grid-cols-2">
        {links.map((item) => (
          <Link key={item.to} to={item.to} className="block">
            <Card className="h-full transition hover:border-foreground/20">
              <div className="flex items-start gap-4">
                <item.icon className="mt-1 size-5 shrink-0 text-muted-foreground" />
                <div>
                  <p className="font-medium text-foreground">{item.label}</p>
                  <p className="mt-1 text-sm text-muted-foreground">{item.description}</p>
                </div>
              </div>
            </Card>
          </Link>
        ))}
      </div>
    </div>
  );
}
