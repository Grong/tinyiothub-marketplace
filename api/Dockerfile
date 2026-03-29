# Build stage
FROM --platform=linux/amd64 rust:latest AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo build --release && rm -rf src

# Copy source and build
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM --platform=linux/amd64 debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/tinyiothub-marketplace-api /app/tinyiothub-marketplace-api

# Create data directory with proper permissions for non-root user
RUN mkdir -p /data && chown -R 1000:1000 /data

# Create non-root user
RUN useradd -m -u 1000 appuser
USER appuser

EXPOSE 3003

ENV PORT=3003
ENV RUST_LOG=info

CMD ["./tinyiothub-marketplace-api"]