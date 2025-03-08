FROM rust:latest as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev libudev-dev cmake build-essential git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Create a new empty project
WORKDIR /app

COPY . .

# Build the project
RUN cargo build --release --bin operator-cli
RUN cargo install --path ./operator-cli --bin operator-cli --locked

# Create a smaller runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev libudev-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/operator-cli /usr/local/bin/operator-cli

COPY --from=builder /app/operator-cli/operators_config.json /usr/local/bin/operators_config.json

COPY --from=builder /app/credentials /usr/local/bin/

# Set the entrypoint
ENTRYPOINT ["operator-cli", "--config-path", "/app/operators_config.json"]
