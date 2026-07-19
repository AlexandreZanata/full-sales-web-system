import { BarChart3, Shield, TrendingUp, Zap } from 'lucide-react';

import { useI18n } from '@/lib/i18n/context';
import { ADMIN_APP_TITLE } from '@/lib/brand';

const FEATURES = [
  { icon: Shield, titleKey: 'auth.featureSecure', descKey: 'auth.featureSecureDesc' },
  { icon: TrendingUp, titleKey: 'auth.featureComplete', descKey: 'auth.featureCompleteDesc' },
  { icon: Zap, titleKey: 'auth.featureFast', descKey: 'auth.featureFastDesc' },
] as const;

export function LoginHero() {
  const { t } = useI18n();

  return (
    <aside className="admin-login-hero relative flex min-h-[28rem] flex-col justify-between overflow-hidden p-8 text-white lg:min-h-full lg:p-12 xl:p-16">
      <div className="admin-login-hero-pattern pointer-events-none absolute inset-0" aria-hidden />
      <div className="relative">
        <div className="flex items-center gap-3">
          <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-admin-login-accent">
            <BarChart3 className="h-5 w-5" aria-hidden />
          </div>
          <p className="text-sm font-semibold uppercase tracking-[0.18em]">{ADMIN_APP_TITLE}</p>
        </div>
        <h1 className="mt-10 max-w-lg text-4xl font-bold leading-tight tracking-tight lg:text-5xl">
          {t('auth.heroTitle')}
          <span className="mt-1 block text-admin-login-accent">{t('auth.heroTitleAccent')}</span>
        </h1>
        <p className="mt-5 max-w-lg text-sm leading-relaxed text-white/75 lg:text-base">
          {t('auth.heroDescription')}
        </p>
      </div>
      <ul className="relative mt-10 grid gap-4 rounded-xl bg-black/25 p-4 backdrop-blur-sm sm:grid-cols-3">
        {FEATURES.map(({ icon: Icon, titleKey, descKey }) => (
          <li key={titleKey} className="space-y-2">
            <Icon className="h-5 w-5 text-admin-login-accent" aria-hidden />
            <p className="text-sm font-semibold">{t(titleKey)}</p>
            <p className="text-xs leading-relaxed text-white/65">{t(descKey)}</p>
          </li>
        ))}
      </ul>
    </aside>
  );
}
