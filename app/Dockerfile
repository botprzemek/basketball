FROM cgr.dev/chainguard/node:latest AS builder

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