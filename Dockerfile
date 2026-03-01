# Multi-stage build for Rust application

# Build stage
FROM rust:1.89-slim AS builder

WORKDIR /usr/src/prompt-sentinel

# Copy all files including Cargo.toml and Cargo.lock
COPY . .

# Build the application in release mode
RUN cargo build --release

# Production stage
FROM ubuntu:24.04

WORKDIR /usr/src/prompt-sentinel

# Install required dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/prompt-sentinel/target/release/prompt_sentinel_server .

# Copy configuration files
COPY config/ ./config/

# Expose the port the app runs on
EXPOSE 3000

# Command to run the application
CMD ["./prompt_sentinel_server"]
