FROM cgr.dev/chainguard/rust:latest AS builder

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