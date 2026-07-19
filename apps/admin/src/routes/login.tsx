import { createFileRoute, redirect, useNavigate } from '@tanstack/react-router';
import { useState } from 'react';

import { useAdminAuth } from '@/auth/useAdminAuth';
import { LoginFormCard } from '@/components/login/LoginFormCard';
import { LoginHero } from '@/components/login/LoginHero';
import { useI18n } from '@/lib/i18n/context';
import { AdminLoginError } from '@/lib/auth/authErrors';
import { readRememberedEmail, writeRememberedEmail } from '@/lib/auth/rememberedEmail';

export const Route = createFileRoute('/login')({
  beforeLoad: async ({ context }) => {
    const user = await context.auth.ensureSession();
    if (user) {
      // TanStack Router redirect helper — not a standard Error
      // eslint-disable-next-line @typescript-eslint/only-throw-error
      throw redirect({ to: '/' });
    }
  },
  component: LoginPage,
});

function LoginPage() {
  const { login, enterDevShell } = useAdminAuth();
  const { t } = useI18n();
  const navigate = useNavigate();
  const remembered = readRememberedEmail();
  const [email, setEmail] = useState(remembered ?? '');
  const [password, setPassword] = useState('');
  const [rememberMe, setRememberMe] = useState(Boolean(remembered));
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit() {
    setError(null);
    setSubmitting(true);
    try {
      await login({ email, password });
      writeRememberedEmail(rememberMe ? email : null);
      void navigate({ to: '/' });
    } catch (err) {
      setError(err instanceof AdminLoginError ? err.message : t('common.unexpectedError'));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="admin-login-screen flex min-h-dvh flex-col">
      <div className="grid flex-1 lg:grid-cols-2">
        <LoginHero />
        <div className="flex items-center justify-center px-4 py-8 sm:px-8">
          <LoginFormCard
            email={email}
            password={password}
            rememberMe={rememberMe}
            error={error}
            submitting={submitting}
            onEmailChange={setEmail}
            onPasswordChange={setPassword}
            onRememberMeChange={setRememberMe}
            onSubmit={handleSubmit}
            onDevEnter={
              import.meta.env.DEV
                ? () => {
                    enterDevShell();
                    void navigate({ to: '/' });
                  }
                : undefined
            }
          />
        </div>
      </div>
    </div>
  );
}
