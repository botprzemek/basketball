ARG VERSION

FROM cgr.dev/chainguard/node:latest AS builder

ARG VERSION=${VERSION}

LABEL org.opencontainers.image.title="auth-app"
LABEL org.opencontainers.image.description="An authorization REST API server featuring RLS database queries, JWT cookies and RBAC. The client implementation includes server-side rendering and navigation guards."
LABEL org.opencontainers.image.base.name="cgr.dev/chainguard/node:latest"
LABEL org.opencontainers.image.authors="https://github.com/botprzemek"
LABEL org.opencontainers.image.url="https://github.com/botprzemek/auth/app"
LABEL org.opencontainers.image.source="https://github.com/botprzemek/auth.git"
LABEL org.opencontainers.image.version="${VERSION}"
WORKDIR /app

COPY --chown=nonroot:nonroot package.json package-lock.json ./
RUN npm ci

COPY --chown=nonroot:nonroot . .
RUN npm run build



FROM cgr.dev/chainguard/node:latest

WORKDIR /app

ENV NODE_ENV=production
ENV NITRO_HOST=0.0.0.0
ENV NITRO_PORT=3000

COPY --from=builder --chown=nonroot:nonroot /app/.output ./.output

EXPOSE 3000

CMD [".output/server/index.mjs"]