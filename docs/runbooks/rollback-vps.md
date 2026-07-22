# Rollback VPS deploy

## App / SPA

1. Keep previous rsync tree or re-checkout last known-good `main` SHA locally.
2. `./production/deploy-to-vps.sh` from that tree (rebuilds SPAs + API image).

## API image only

```bash
ssh root@YOUR_VPS_IP
cd /var/www/fullsales
docker compose -p fullsales -f infra/docker-compose.prod.yml \
  --env-file production/env/docker.env images
# rebuild previous commit after checkout, or tag images before upgrade
docker compose -p fullsales -f infra/docker-compose.prod.yml \
  --env-file production/env/docker.env up -d api
```

## Data

Postgres/Redis/MinIO volumes are **not** deleted by deploy. Avoid `docker compose down -v`.

Backup before risky migrations:

```bash
docker compose -p fullsales -f infra/docker-compose.prod.yml \
  --env-file production/env/docker.env exec -T postgres \
  pg_dump -U fullsales fullsales_prod > fullsales-$(date +%F).sql
```

## Nginx

```bash
# Re-enable previous conf if needed; do not remove other sites
ls /etc/nginx/sites-enabled/
nginx -t && systemctl reload nginx
```
