import { ShoppingBag } from 'lucide-react';

type ProductCardAddPillProps = {
  label: string;
  ariaLabel: string;
  onClick: () => void;
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
