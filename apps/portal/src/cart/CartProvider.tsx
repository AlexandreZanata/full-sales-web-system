import { createContext, useCallback, useContext, useMemo, useState, type ReactNode } from 'react';

import type { PortalProduct } from '@/lib/api/types';

export type CartLine = {
  productId: string;
  name: string;
  sku: string;
  unitPriceAmount: number;
  unitPriceCurrency: string;
  quantity: number;
  primaryImageUrl?: string;
};

type CartContextValue = {
  lines: CartLine[];
  itemCount: number;
  totalAmount: number;
  currency: string;
  addProduct: (product: PortalProduct, quantity?: number) => void;
  setQuantity: (productId: string, quantity: number) => void;
  removeLine: (productId: string) => void;
  clear: () => void;
};

const CartContext = createContext<CartContextValue | null>(null);

export function CartProvider({ children }: { children: ReactNode }) {
  const [lines, setLines] = useState<CartLine[]>([]);

  const addProduct = useCallback((product: PortalProduct, quantity = 1) => {
    setLines((current) => {
      const existing = current.find((line) => line.productId === product.id);
      if (existing) {
        return current.map((line) =>
          line.productId === product.id ? { ...line, quantity: line.quantity + quantity } : line,
        );
      }
      return [
        ...current,
        {
          productId: product.id,
          name: product.name,
          sku: product.sku,
          unitPriceAmount: product.priceAmount,
          unitPriceCurrency: product.priceCurrency,
          quantity,
          primaryImageUrl: product.primaryImageUrl,
        },
      ];
    });
  }, []);

  const setQuantity = useCallback((productId: string, quantity: number) => {
    if (quantity <= 0) {
      setLines((current) => current.filter((line) => line.productId !== productId));
      return;
    }
    setLines((current) =>
      current.map((line) => (line.productId === productId ? { ...line, quantity } : line)),
    );
  }, []);

  const removeLine = useCallback((productId: string) => {
    setLines((current) => current.filter((line) => line.productId !== productId));
  }, []);

  const clear = useCallback(() => {
    setLines([]);
  }, []);

  const value = useMemo<CartContextValue>(() => {
    const itemCount = lines.reduce((sum, line) => sum + line.quantity, 0);
    const totalAmount = lines.reduce((sum, line) => sum + line.unitPriceAmount * line.quantity, 0);
    const currency = lines[0]?.unitPriceCurrency ?? 'BRL';
    return { lines, itemCount, totalAmount, currency, addProduct, setQuantity, removeLine, clear };
  }, [lines, addProduct, setQuantity, removeLine, clear]);

  return <CartContext.Provider value={value}>{children}</CartContext.Provider>;
}

export function useCart(): CartContextValue {
  const context = useContext(CartContext);
  if (!context) {
    throw new Error('useCart must be used within CartProvider');
  }
  return context;
}
