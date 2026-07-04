import { createFileRoute, useNavigate } from '@tanstack/react-router';

import { CreateUserForm } from '@/components/users/CreateUserForm';
import { PageBackLink } from '@/components/ui/PageBackLink';
import { PageHeader } from '@/components/ui/PageHeader';
import { createUser } from '@/lib/api/users';

export const Route = createFileRoute('/_authenticated/users/new')({
  component: NewUserPage,
});

function NewUserPage() {
  const navigate = useNavigate();

  return (
    <div>
      <PageHeader
        title="New user"
        description="Create an account and assign a role."
        back={<PageBackLink label="Back to users" to="/users" />}
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
