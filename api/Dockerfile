FROM cgr.dev/chainguard/rust:latest AS builder

WORKDIR /app

COPY --chown=nonroot:nonroot Cargo.toml Cargo.lock ./

RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY --chown=nonroot:nonroot . .

RUN cargo build --release --bin api_core



FROM cgr.dev/chainguard/glibc-dynamic AS runner

WORKDIR /app

COPY --from=builder --chown=nonroot:nonroot /app/target/release/api_core /usr/local/bin/api_core

EXPOSE 3000

CMD ["/usr/local/bin/api_core"]