# Stage 1: Build
FROM rust:1.76-slim as builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim as runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
RUN useradd -u 1001 -m potion
WORKDIR /app
COPY --from=builder /app/target/release/potion-shop-axum-k8s ./
COPY static ./static
USER potion
EXPOSE 3000
CMD ["./potion-shop-axum-k8s"]
