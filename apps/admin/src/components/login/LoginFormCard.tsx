import { ArrowRight, Eye, EyeOff, Lock, Terminal, User } from 'lucide-react';
import { useState } from 'react';

import { LocaleSwitcher } from '@/components/LocaleSwitcher';
import { LoginTextField } from '@/components/login/LoginTextField';
import { Button } from '@/components/ui/Button';
import { useI18n } from '@/lib/i18n/context';
import { ADMIN_APP_TITLE } from '@/lib/brand';

type LoginFormCardProps = {
  email: string;
  password: string;
  rememberMe: boolean;
  error: string | null;
  submitting: boolean;
  onEmailChange: (value: string) => void;
  onPasswordChange: (value: string) => void;
  onRememberMeChange: (value: boolean) => void;
  onSubmit: () => void | Promise<void>;
  onDevEnter?: () => void;
};

export function LoginFormCard({
  email,
  password,
  rememberMe,
  error,
  submitting,
  onEmailChange,
  onPasswordChange,
  onRememberMeChange,
  onSubmit,
  onDevEnter,
}: LoginFormCardProps) {
  const { t } = useI18n();
  const [showPassword, setShowPassword] = useState(false);

  return (
    <div className="admin-login-card w-full max-w-md rounded-2xl border border-hairline bg-surface p-6 shadow-xl shadow-black/5 sm:p-8">
      <div className="mb-6 flex justify-end">
        <LocaleSwitcher className="admin-login-locale" />
      </div>
      <p className="text-xs font-semibold uppercase tracking-[0.2em] text-admin-login-accent">
        {ADMIN_APP_TITLE}
      </p>
      <h2 className="mt-2 text-3xl font-bold tracking-tight text-foreground">
        {t('auth.signInTitle')}
      </h2>
      <p className="mt-2 text-sm text-muted-foreground">{t('auth.signInDescription')}</p>

      <form
        className="mt-8 space-y-4"
        onSubmit={(event) => {
          event.preventDefault();
          void onSubmit();
        }}
      >
        <LoginTextField
          label={t('auth.email')}
          name="email"
          type="email"
          autoComplete="username"
          required
          value={email}
          leftIcon={<User className="h-4 w-4" aria-hidden />}
          onChange={(event) => {
            onEmailChange(event.target.value);
          }}
        />
        <LoginTextField
          label={t('auth.password')}
          name="password"
          type={showPassword ? 'text' : 'password'}
          autoComplete="current-password"
          required
          value={password}
          leftIcon={<Lock className="h-4 w-4" aria-hidden />}
          rightSlot={
            <button
              type="button"
              className="rounded-md p-1.5 text-muted-foreground hover:bg-surface-muted hover:text-foreground"
              aria-label={showPassword ? t('auth.hidePassword') : t('auth.showPassword')}
              onClick={() => {
                setShowPassword((value) => !value);
              }}
            >
              {showPassword ? (
                <EyeOff className="h-4 w-4" aria-hidden />
              ) : (
                <Eye className="h-4 w-4" aria-hidden />
              )}
            </button>
          }
          onChange={(event) => {
            onPasswordChange(event.target.value);
          }}
        />

        <div className="flex flex-wrap items-center justify-between gap-3 pt-1">
          <label className="inline-flex items-center gap-2 text-sm text-foreground">
            <input
              type="checkbox"
              className="h-4 w-4 rounded border-input text-admin-login-accent focus:ring-admin-login-accent"
              checked={rememberMe}
              onChange={(event) => {
                onRememberMeChange(event.target.checked);
              }}
            />
            {t('auth.rememberMe')}
          </label>
          <span className="text-sm font-medium text-admin-login-accent">
            {t('auth.forgotPassword')}
          </span>
        </div>

        {error ? (
          <p className="rounded-md border border-destructive/30 bg-destructive/5 px-3 py-2 text-sm text-destructive">
            {error}
          </p>
        ) : null}

        <Button
          type="submit"
          className="h-11 w-full gap-2 bg-admin-login-accent text-white hover:bg-admin-login-accent/90"
          disabled={submitting}
        >
          {submitting ? t('auth.signingIn') : t('auth.signIn')}
          {!submitting ? <ArrowRight className="h-4 w-4" aria-hidden /> : null}
        </Button>
      </form>

      {onDevEnter ? (
        <>
          <div className="my-6 flex items-center gap-3 text-xs font-semibold uppercase tracking-[0.16em] text-muted-foreground">
            <span className="h-px flex-1 bg-hairline" />
            {t('auth.orDivider')}
            <span className="h-px flex-1 bg-hairline" />
          </div>
          <Button
            type="button"
            variant="secondary"
            className="h-11 w-full gap-2"
            onClick={onDevEnter}
          >
            <Terminal className="h-4 w-4" aria-hidden />
            {t('auth.devEnter')}
          </Button>
        </>
      ) : null}
    </div>
  );
}
