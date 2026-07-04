import { Link, type LinkProps } from '@tanstack/react-router';

import { Button } from '@/components/ui/Button';

type PageBackLinkProps = {
  label: string;
} & Pick<LinkProps, 'to' | 'search' | 'params'>;

export function PageBackLink({ label, to, search, params }: PageBackLinkProps) {
  return (
    <Link to={to} search={search} params={params}>
      <Button>{label}</Button>
    </Link>
  );
}
