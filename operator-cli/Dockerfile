FROM rust:latest as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev libudev-dev cmake build-essential git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Create a new empty project
WORKDIR /app

COPY . .

# Copy your operator_cli code if it's a local dependency
# Uncomment and adjust path as needed:
# COPY ./operator_cli /app/operator_cli

# Build dependencies - this is the caching Docker layer
RUN cargo build --release

# Create a smaller runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev libudev-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/operator-cli/target/release/operator-cli /app/operator-cli

COPY --from=builder /app/operator-cli/operators_config.json /app/operators_config.json

COPY --from=builder /app/credentials /app/

# Create a directory for keypairs
RUN mkdir -p /app/keypairs

# Set the entrypoint
ENTRYPOINT ["/app/operator-cli", "--config-path", "/app/operators_config.json"]
