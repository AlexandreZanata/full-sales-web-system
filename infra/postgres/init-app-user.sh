#!/bin/sh
# Runs once on empty Postgres volume — creates app_user before sqlx migrations.
set -eu

psql -v ON_ERROR_STOP=1 --username "${POSTGRES_USER}" --dbname "${POSTGRES_DB}" <<EOSQL
DO \$\$
BEGIN
  IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'app_user') THEN
    CREATE ROLE app_user LOGIN PASSWORD '${APP_USER_PASSWORD}' NOSUPERUSER NOBYPASSRLS;
  END IF;
END
\$\$;
EOSQL
