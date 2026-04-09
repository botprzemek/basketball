FROM cgr.dev/chainguard/rust:latest AS builder

COPY --chown=nonroot:nonroot Cargo.toml Cargo.lock ./
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

WORKDIR /api
COPY --chown=nonroot:nonroot . .
RUN cargo build --release --bin api



FROM cgr.dev/chainguard/glibc-dynamic AS runner

WORKDIR /api

ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000

COPY --from=builder --chown=nonroot:nonroot /api/target/release/api /usr/local/bin/api

EXPOSE 3000

CMD ["/usr/local/bin/api"]