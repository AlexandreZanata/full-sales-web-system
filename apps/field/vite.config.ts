import tailwindcss from '@tailwindcss/vite';
import { tanstackRouter } from '@tanstack/router-plugin/vite';
import react from '@vitejs/plugin-react';
import { defineConfig, loadEnv } from 'vite';
import tsconfigPaths from 'vite-tsconfig-paths';

const API_ORIGIN = process.env.VITE_DEV_API_ORIGIN ?? 'http://127.0.0.1:8080';

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '');
  return {
    base: env.VITE_BASE || '/',
    plugins: [
      tanstackRouter({ target: 'react', autoCodeSplitting: true }),
      tsconfigPaths(),
      tailwindcss(),
      react(),
    ],
    server: {
      host: '127.0.0.1',
      port: 5176,
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
  };
});
