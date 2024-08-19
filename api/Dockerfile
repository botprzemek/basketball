# Build a production distribution

FROM cgr.dev/chainguard/node:latest AS setup

WORKDIR /app

COPY --chown=node:node package*.json .

RUN npm install --clean

COPY --chown=node:node . .

RUN npm run build

# Setup production dependecies

FROM setup AS production

WORKDIR /app

COPY --chown=node:node package*.json .

ENV NODE_ENV=production

RUN npm install --clean --production

COPY --from=setup --chown=node:node /app/dist ./dist

RUN npm prune --production

# Create a clean environment

FROM alpine AS runner

LABEL authors="botprzemek"

RUN apk add --update nodejs

WORKDIR /app

COPY --from=production --chown=node:node /app/dist ./dist
COPY --from=production --chown=node:node /app/node_modules ./node_modules
COPY --from=production --chown=node:node /app/package.json ./package.json

ENV NODE_ENV=production

CMD ["node", "./dist/index.js"]

EXPOSE 3000