# Build context: repository root
# Example:
#   docker build -f deploy/docker/spa.Dockerfile \
#     --build-arg APP_NAME=admin \
#     --build-arg VITE_API_BASE_URL=https://api.example.com/v1 \
#     -t fullsales-admin:local .

ARG APP_NAME=admin
ARG VITE_API_BASE_URL=/v1

FROM node:22-bookworm-slim AS build
ARG APP_NAME
ARG VITE_API_BASE_URL
WORKDIR /src

RUN corepack enable && corepack prepare pnpm@9.15.9 --activate

COPY package.json pnpm-workspace.yaml pnpm-lock.yaml tsconfig.base.json ./
COPY packages ./packages
COPY apps/admin/package.json apps/admin/
COPY apps/portal/package.json apps/portal/
COPY apps/platform-admin/package.json apps/platform-admin/
COPY apps/field/package.json apps/field/
COPY apps/web/package.json apps/web/
COPY apps/api/package.json apps/api/

RUN pnpm install --filter "@full-sales/${APP_NAME}" --frozen-lockfile

COPY apps/${APP_NAME} apps/${APP_NAME}

ENV VITE_API_BASE_URL=${VITE_API_BASE_URL}
RUN pnpm --filter "@full-sales/${APP_NAME}" build

FROM nginx:1.27-alpine AS runtime
ARG APP_NAME
COPY deploy/docker/spa.nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /src/apps/${APP_NAME}/dist /usr/share/nginx/html
EXPOSE 80
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget -q -O /dev/null http://127.0.0.1/ || exit 1
