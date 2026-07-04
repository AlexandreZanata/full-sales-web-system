import { useEffect, useState } from 'react';

const defaultApiUrl = 'http://127.0.0.1:8080';
const apiUrl = import.meta.env.VITE_API_URL ?? defaultApiUrl;

interface HealthResponse {
  status: string;
}

export function App() {
  const [health, setHealth] = useState<string>('checking…');

  useEffect(() => {
    void fetch(`${apiUrl}/health`)
      .then(async (res) => {
        if (!res.ok) {
          setHealth(`error ${String(res.status)}`);
          return;
        }
        const body = (await res.json()) as HealthResponse;
        setHealth(body.status);
      })
      .catch(() => {
        setHealth('unreachable');
      });
  }, []);

  return (
    <main style={{ fontFamily: 'system-ui, sans-serif', padding: '2rem', maxWidth: '40rem' }}>
      <h1>Full Sales Web System</h1>
      <p>Driver/seller control — inventory, commerces, sales, signed reports.</p>
      <p>
        API health: <strong>{health}</strong> ({apiUrl}/health)
      </p>
      <p>
        <a href="https://github.com/AlexandreZanata/full-sales-web-system">Documentation</a>
      </p>
    </main>
  );
}
