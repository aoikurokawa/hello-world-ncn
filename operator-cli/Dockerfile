FROM rust:1.70-slim-bullseye as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev libudev-dev cmake build-essential git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Create a new empty project
WORKDIR /app
RUN USER=root cargo new --bin operator-client
WORKDIR /app/operator-client

# Copy over your manifests
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./operators_config.json ./

# Copy your source code
COPY ./src ./src

# Copy your operator_cli code if it's a local dependency
# Uncomment and adjust path as needed:
# COPY ./operator_cli /app/operator_cli

# Build dependencies - this is the caching Docker layer
RUN cargo build --release
RUN rm src/*.rs

# Build your application
COPY ./src ./src
RUN cargo build --release

# Create a smaller runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev libudev-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/operator-client/target/release/operator-client /app/operator-client

# Copy the config file
COPY --from=builder /app/operator-client/operators_config.json /app/operators_config.json

# Create a directory for keypairs
RUN mkdir -p /app/keypairs

# Set the entrypoint
ENTRYPOINT ["/app/operator-client", "--config-path", "/app/operators_config.json"]
