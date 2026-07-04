import {
  ClipboardList,
  FileBarChart,
  LayoutDashboard,
  Package,
  ScrollText,
  Settings,
  ShoppingCart,
  Store,
  Truck,
  Users,
  Warehouse,
  type LucideIcon,
} from 'lucide-react';

import type { MessageKey } from '@/lib/i18n/messages';

export const adminNavItems = [
  { to: '/', labelKey: 'nav.dashboard', icon: LayoutDashboard },
  { to: '/users', labelKey: 'nav.users', icon: Users },
  { to: '/commerces', labelKey: 'nav.commerces', icon: Store },
  { to: '/products', labelKey: 'nav.products', icon: Package },
  { to: '/inventory', labelKey: 'nav.inventory', icon: Warehouse },
  { to: '/orders', labelKey: 'nav.orders', icon: ShoppingCart },
  { to: '/deliveries', labelKey: 'nav.deliveries', icon: Truck },
  { to: '/sales', labelKey: 'nav.sales', icon: ClipboardList },
  { to: '/reports', labelKey: 'nav.reports', icon: FileBarChart },
  { to: '/audit', labelKey: 'nav.audit', icon: ScrollText },
  { to: '/settings', labelKey: 'nav.settings', icon: Settings },
] as const satisfies ReadonlyArray<{
  to: string;
  labelKey: MessageKey;
  icon: LucideIcon;
}>;
