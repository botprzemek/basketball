FROM cgr.dev/chainguard/rust:latest AS builder

COPY --chown=nonroot:nonroot Cargo.toml Cargo.lock ./
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

WORKDIR /app
COPY --chown=nonroot:nonroot . .
RUN cargo build --release --bin auth_api



FROM cgr.dev/chainguard/glibc-dynamic AS runner

WORKDIR /app

ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000

COPY --from=builder --chown=nonroot:nonroot /app/target/release/auth_api /usr/local/bin/auth_api

EXPOSE 3000

CMD ["/usr/local/bin/auth_api"]