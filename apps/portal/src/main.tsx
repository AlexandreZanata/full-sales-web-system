import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';

import { App } from './App';
import './styles/theme.css';
import './styles/app-chrome.css';
import './styles/catalog.css';
import './styles/cart.css';

const rootElement = document.getElementById('root');
if (!rootElement) {
  throw new Error('Root element #root not found');
}

if ('serviceWorker' in navigator && import.meta.env.PROD) {
  void navigator.serviceWorker.register('/sw.js');
}

createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
