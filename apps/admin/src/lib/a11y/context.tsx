import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from 'react';

import {
  applyTextSizePresetToDocument,
  resolveInitialTextSizePreset,
  writeStoredTextSizePreset,
} from '@/lib/a11y/storage';
import type { TextSizePreset } from '@/lib/a11y/types';

type AccessibilityContextValue = {
  textSizePreset: TextSizePreset;
  setTextSizePreset: (preset: TextSizePreset) => void;
};

const AccessibilityContext = createContext<AccessibilityContextValue | null>(null);

export function AccessibilityProvider({ children }: { children: ReactNode }) {
  const [textSizePreset, setTextSizePresetState] = useState<TextSizePreset>(() =>
    resolveInitialTextSizePreset(),
  );

  useEffect(() => {
    applyTextSizePresetToDocument(textSizePreset);
  }, [textSizePreset]);

  const setTextSizePreset = useCallback((next: TextSizePreset) => {
    writeStoredTextSizePreset(next);
    setTextSizePresetState(next);
  }, []);

  const value = useMemo(
    () => ({ textSizePreset, setTextSizePreset }),
    [textSizePreset, setTextSizePreset],
  );

  return <AccessibilityContext.Provider value={value}>{children}</AccessibilityContext.Provider>;
}

export function useAccessibility(): AccessibilityContextValue {
  const context = useContext(AccessibilityContext);
  if (!context) {
    throw new Error('useAccessibility must be used within AccessibilityProvider');
  }
  return context;
}
