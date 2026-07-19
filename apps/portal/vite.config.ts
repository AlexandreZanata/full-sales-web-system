import tailwindcss from '@tailwindcss/vite';
import { tanstackRouter } from '@tanstack/router-plugin/vite';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import tsconfigPaths from 'vite-tsconfig-paths';

const API_ORIGIN = process.env.VITE_DEV_API_ORIGIN ?? 'http://127.0.0.1:8080';

export default defineConfig({
  plugins: [
    tanstackRouter({ target: 'react', autoCodeSplitting: true }),
    tsconfigPaths(),
    tailwindcss(),
    react(),
  ],
  server: {
    // LAN bind so seller share links (http://<host-ip>:5175/s/…) work on real devices.
    host: true,
    port: 5175,
    strictPort: true,
    proxy: {
      '/v1': {
        target: API_ORIGIN,
        changeOrigin: true,
      },
      '/health': {
        target: API_ORIGIN,
        changeOrigin: true,
      },
    },
  },
});
