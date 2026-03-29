# Build stage
FROM rust:1.75-alpine AS builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

# Copy manifests
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo build --release && rm -rf src

# Copy source and build
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM alpine:3.19

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates openssl

# Copy binary from builder
COPY --from=builder /app/target/release/tinyiothub-marketplace-api /app/tinyiothub-marketplace-api

# Create non-root user
RUN adduser -D -u 1000 appuser
USER appuser

EXPOSE 3003

ENV PORT=3003
ENV RUST_LOG=info

CMD ["./tinyiothub-marketplace-api"]
