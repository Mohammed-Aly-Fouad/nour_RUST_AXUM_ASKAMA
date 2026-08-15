# Stage 1: Build the binary
FROM rust:1.88-slim AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential && rm -rf /var/lib/apt/lists/*

# Cache dependencies separately from source changes
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release --bin nour \
    && rm -rf src

# Now copy actual source (migrations included automatically)
COPY . .

ENV SQLX_OFFLINE=1
RUN cargo build --release --bin nour

# Stage 2: Runtime image
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1000 appuser

COPY --from=builder /app/target/release/nour /app/nour
# COPY --from=builder /app/templates /app/templates
# Copy static assets and templates used at runtime
COPY --from=builder /app/assets /app/assets
COPY --from=builder /app/templates /app/templates

USER appuser
ENV SERVER_ADDRESS=0.0.0.0:8080
EXPOSE 8080
CMD ["./nour"]