import {
  Activity,
  CreditCard,
  Globe,
  LayoutDashboard,
  ScrollText,
  ShieldAlert,
  Users,
  Wrench,
  Building2,
  type LucideIcon,
} from 'lucide-react';

import type { MessageKey } from '@/lib/i18n/messages';

export const platformNavItems = [
  { to: '/', labelKey: 'nav.dashboard', icon: LayoutDashboard },
  { to: '/tenants', labelKey: 'nav.tenants', icon: Building2 },
  { to: '/users', labelKey: 'nav.users', icon: Users },
  { to: '/billing', labelKey: 'nav.billing', icon: CreditCard },
  { to: '/fraud', labelKey: 'nav.fraud', icon: ShieldAlert },
  { to: '/domains', labelKey: 'nav.domains', icon: Globe },
  { to: '/health', labelKey: 'nav.health', icon: Activity },
  { to: '/maintenance', labelKey: 'nav.maintenance', icon: Wrench },
  { to: '/audit', labelKey: 'nav.audit', icon: ScrollText },
] as const satisfies ReadonlyArray<{
  to: string;
  labelKey: MessageKey;
  icon: LucideIcon;
}>;
