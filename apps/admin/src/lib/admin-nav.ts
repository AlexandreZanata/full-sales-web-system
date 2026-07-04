import {
  ClipboardList,
  FileBarChart,
  LayoutDashboard,
  Package,
  ScrollText,
  ShoppingCart,
  Store,
  Truck,
  Users,
  Warehouse,
  type LucideIcon,
} from 'lucide-react';

export const adminNavItems = [
  { to: '/', label: 'Dashboard', icon: LayoutDashboard },
  { to: '/users', label: 'Users', icon: Users },
  { to: '/commerces', label: 'Commerces', icon: Store },
  { to: '/products', label: 'Products', icon: Package },
  { to: '/inventory', label: 'Inventory', icon: Warehouse },
  { to: '/orders', label: 'Orders', icon: ShoppingCart },
  { to: '/deliveries', label: 'Deliveries', icon: Truck },
  { to: '/sales', label: 'Sales', icon: ClipboardList },
  { to: '/reports', label: 'Reports', icon: FileBarChart },
  { to: '/audit', label: 'Audit', icon: ScrollText },
] as const satisfies ReadonlyArray<{
  to: string;
  label: string;
  icon: LucideIcon;
}>;
