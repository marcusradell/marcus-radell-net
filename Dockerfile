# Build stage
FROM rust:slim-bullseye AS builder

WORKDIR /usr/src/app

# Copy over manifests
COPY Cargo.lock Cargo.toml ./

# Create a dummy src/main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {println!(\"dummy\");}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY ./src ./src

# Build the actual application
RUN cargo build --release

# Production stage
FROM debian:bullseye-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-privileged user
RUN groupadd -r docker && useradd -r -g docker docker

# Copy the build artifact from the build stage
COPY --from=builder /usr/src/app/target/release/marcus-radell-net /usr/local/bin/

# Set the user to run the application
USER docker

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/marcus-radell-net"]

# Expose port if your application listens on a port
EXPOSE 8080

