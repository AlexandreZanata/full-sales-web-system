import { ShoppingBag } from 'lucide-react';
import type { MouseEvent } from 'react';

type ProductCardAddPillProps = {
  label: string;
  ariaLabel: string;
  onClick: (event: MouseEvent<HTMLButtonElement>) => void;
};

export function ProductCardAddPill({ label, ariaLabel, onClick }: ProductCardAddPillProps) {
  return (
    <button
      type="button"
      className="catalog-add-pill-btn shrink-0"
      aria-label={ariaLabel}
      onClick={onClick}
    >
      <ShoppingBag className="size-3.5" aria-hidden />
      <span>{label}</span>
    </button>
  );
}
