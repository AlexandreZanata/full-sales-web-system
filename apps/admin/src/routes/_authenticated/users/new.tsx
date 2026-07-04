import { createFileRoute, useNavigate } from '@tanstack/react-router';

import { CreateUserForm } from '@/components/users/CreateUserForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { createUser } from '@/lib/api/users';
import { useI18n } from '@/lib/i18n/context';

export const Route = createFileRoute('/_authenticated/users/new')({
  component: NewUserPage,
});

function NewUserPage() {
  const navigate = useNavigate();
  const { t } = useI18n();

  return (
    <div>
      <PageHeader
        title={t('users.create.title')}
        description={t('users.create.description')}
        back={<PageBackLink label={t('common.backTo.users')} to="/users" />}
      />

      <CreateUserForm
        onSubmit={createUser}
        onSuccess={(user) => {
          void navigate({ to: '/users/$id', params: { id: user.id } });
        }}
      />
    </div>
  );
}
