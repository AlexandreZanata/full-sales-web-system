import { createFileRoute, redirect, useNavigate } from '@tanstack/react-router';
import { useState, type SubmitEvent } from 'react';

import { usePlatformAuth } from '@/auth/usePlatformAuth';
import { BrandMark } from '@/components/BrandMark';
import { LocaleSwitcher } from '@/components/LocaleSwitcher';
import { Button } from '@/components/ui/Button';
import { Card } from '@/components/ui/Card';
import { Input } from '@/components/ui/Input';
import { PlatformLoginError } from '@/lib/auth/authErrors';
import { PLATFORM_APP_TAGLINE, PLATFORM_APP_TITLE, PLATFORM_BRAND_NAME } from '@/lib/brand';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/login')({
  beforeLoad: async ({ context }) => {
    const user = await context.auth.ensureSession();
    if (user) {
      // eslint-disable-next-line @typescript-eslint/only-throw-error
      throw redirect({ to: '/' });
    }
  },
  component: LoginPage,
});

function LoginPage() {
  const { login, verifyMfa, enterDevShell } = usePlatformAuth();
  const { t } = useI18n();
  const navigate = useNavigate();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [mfaToken, setMfaToken] = useState<string | null>(null);
  const [mfaCode, setMfaCode] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  async function handleLogin(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    setError(null);
    setSubmitting(true);
    try {
      const result = await login({ email, password });
      if (result.kind === 'mfa') {
        setMfaToken(result.mfaToken);
      } else {
        void navigate({ to: '/' });
      }
    } catch (err) {
      setError(err instanceof PlatformLoginError ? err.message : t('common.unexpectedError'));
    } finally {
      setSubmitting(false);
    }
  }

  async function handleMfa(event: SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!mfaToken) {
      return;
    }
    setError(null);
    setSubmitting(true);
    try {
      await verifyMfa({ code: mfaCode, mfaToken });
      void navigate({ to: '/' });
    } catch (err) {
      setError(err instanceof PlatformLoginError ? err.message : t('common.unexpectedError'));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="platform-login-screen flex min-h-dvh items-center justify-center px-4 py-8">
      <Card className="platform-login-card w-full max-w-md border bg-surface/95 backdrop-blur-sm">
        <div className="mb-4 flex items-start justify-between gap-3">
          <BrandMark
            size="lg"
            variant="login"
            className="!bg-primary !text-primary-foreground !ring-0"
          />
          <LocaleSwitcher />
        </div>
        <div className="mb-6">
          <p className="platform-login-kicker text-xs font-bold uppercase tracking-[0.25em]">
            {PLATFORM_BRAND_NAME}
          </p>
          <h1 className="mt-2 text-2xl font-semibold tracking-tight">
            {mfaToken ? t('auth.mfaTitle') : PLATFORM_APP_TITLE}
          </h1>
          <p className="mt-1 text-sm text-muted-foreground">
            {mfaToken ? t('auth.mfaDescription') : PLATFORM_APP_TAGLINE}
          </p>
        </div>

        {mfaToken ? (
          <form className="space-y-4" onSubmit={(e) => void handleMfa(e)}>
            <Input
              label={t('auth.mfaCode')}
              name="code"
              inputMode="numeric"
              required
              value={mfaCode}
              onChange={(e) => {
                setMfaCode(e.target.value);
              }}
            />
            {error ? <p className="text-sm text-destructive">{error}</p> : null}
            <Button type="submit" className="w-full" disabled={submitting}>
              {t('auth.verifyMfa')}
            </Button>
          </form>
        ) : (
          <form className="space-y-4" onSubmit={(e) => void handleLogin(e)}>
            <Input
              label={t('auth.email')}
              name="email"
              type="email"
              required
              value={email}
              onChange={(e) => {
                setEmail(e.target.value);
              }}
            />
            <Input
              label={t('auth.password')}
              name="password"
              type="password"
              required
              value={password}
              onChange={(e) => {
                setPassword(e.target.value);
              }}
            />
            {error ? <p className="text-sm text-destructive">{error}</p> : null}
            <Button
              type="submit"
              className="w-full shadow-md shadow-primary/25"
              disabled={submitting}
            >
              {submitting ? t('auth.signingIn') : t('auth.signIn')}
            </Button>
          </form>
        )}

        {import.meta.env.DEV && !mfaToken ? (
          <div className="mt-4 border-t border-hairline pt-4">
            <Button
              type="button"
              variant="secondary"
              className="w-full"
              onClick={() => {
                enterDevShell();
                void navigate({ to: '/' });
              }}
            >
              {t('auth.devEnter')}
            </Button>
          </div>
        ) : null}
      </Card>
    </div>
  );
}
