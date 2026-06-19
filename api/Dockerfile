FROM cgr.dev/chainguard/rust:latest AS builder

WORKDIR /app

COPY --chown=nonroot:nonroot Cargo.toml Cargo.lock ./

RUN mkdir src
RUN echo "fn main() { println!(\"hello_world\")}" > src/main.rs
RUN cargo build --release

COPY --chown=nonroot:nonroot ./src ./src

RUN cargo build --release --bin auth-api



FROM ubuntu AS runner

WORKDIR /app

COPY --from=builder --chown=nonroot:nonroot /app/target/release/auth-api /usr/local/bin/auth-api

EXPOSE 3000

CMD ["/usr/local/bin/auth-api"]