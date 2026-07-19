type CartQuantityStepperProps = {
  quantity: number;
  decreaseLabel: string;
  increaseLabel: string;
  quantityLabel: string;
  onDecrease: () => void;
  onIncrease: () => void;
};

export function CartQuantityStepper({
  quantity,
  decreaseLabel,
  increaseLabel,
  quantityLabel,
  onDecrease,
  onIncrease,
}: CartQuantityStepperProps) {
  return (
    <div className="cart-qty-stepper" role="group" aria-label={quantityLabel}>
      <button
        type="button"
        className="cart-qty-stepper__btn"
        aria-label={decreaseLabel}
        disabled={quantity <= 1}
        onClick={onDecrease}
      >
        −
      </button>
      <span className="cart-qty-stepper__value" aria-live="polite">
        {quantity}
      </span>
      <button
        type="button"
        className="cart-qty-stepper__btn"
        aria-label={increaseLabel}
        onClick={onIncrease}
      >
        +
      </button>
    </div>
  );
}
