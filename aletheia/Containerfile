# Multi-stage Containerfile for Aletheia
# Produces a minimal, secure container image

# Stage 1: Build
FROM rust:1.85-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build release binary (statically linked)
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Runtime
FROM scratch

# Copy binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/aletheia /aletheia

# Set entrypoint
ENTRYPOINT ["/aletheia"]

# Default to verifying /repo
CMD ["/repo"]

# Labels
LABEL org.opencontainers.image.title="Aletheia"
LABEL org.opencontainers.image.description="RSR compliance verification tool"
LABEL org.opencontainers.image.version="0.1.0"
LABEL org.opencontainers.image.authors="MAA Framework"
LABEL org.opencontainers.image.source="https://gitlab.com/maa-framework/6-the-foundation/aletheia"
LABEL org.opencontainers.image.licenses="MIT OR Palimpsest-0.8"

# Usage:
# Build: docker build -t aletheia:0.1.0 .
# Run: docker run -v /path/to/repo:/repo aletheia:0.1.0
