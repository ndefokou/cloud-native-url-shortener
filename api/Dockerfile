# ===========================================
# Stage 1: Build stage
# ===========================================
FROM rust:1.85-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Create working directory
WORKDIR /app

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock* ./

# Create dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the application
RUN touch src/main.rs && \
    cargo build --release

# ===========================================
# Stage 2: Runtime stage
# ===========================================
FROM alpine:3.19 AS runtime

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata && \
    # Create non-root user for security
    addgroup -g 1000 appgroup && \
    adduser -u 1000 -G appgroup -D appuser

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/url-shortener /app/url-shortener

# Set ownership to non-root user
RUN chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Set environment variables
ENV HOST=0.0.0.0
ENV PORT=8080
ENV RUST_LOG=info

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://127.0.0.1:8080/health || exit 1

# Run the binary
ENTRYPOINT ["/app/url-shortener"]