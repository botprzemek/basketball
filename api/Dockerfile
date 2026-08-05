ARG VERSION

FROM cgr.dev/chainguard/rust:latest AS builder

ARG VERSION=${VERSION}

LABEL org.opencontainers.image.title="auth-api"
LABEL org.opencontainers.image.description="An authorization REST API server featuring RLS database queries, JWT cookies and RBAC. The client implementation includes server-side rendering and navigation guards."
LABEL org.opencontainers.image.base.name="cgr.dev/chainguard/rust:latest"
LABEL org.opencontainers.image.authors="https://github.com/botprzemek"
LABEL org.opencontainers.image.url="https://github.com/botprzemek/auth/api"
LABEL org.opencontainers.image.source="https://github.com/botprzemek/auth.git"
LABEL org.opencontainers.image.version="${VERSION}"

COPY --chown=nonroot:nonroot Cargo.toml Cargo.lock ./
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --locked --release

WORKDIR /app
COPY --chown=nonroot:nonroot . .
RUN cargo build --locked --release --bin auth-api



FROM cgr.dev/chainguard/glibc-dynamic AS runner

WORKDIR /app

COPY --from=builder --chown=nonroot:nonroot /app/target/release/auth-api /bin/auth-api

EXPOSE 3000

CMD ["/bin/auth-api"]