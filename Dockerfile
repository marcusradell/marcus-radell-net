# Build stage
FROM rust:alpine AS builder

WORKDIR /usr/src/app

RUN apk add pkgconfig openssl-dev libc-dev

# Copy the entire project
COPY . .

RUN cargo build --release

# Production stage - using Alpine for minimal image size
FROM rust:alpine

RUN apk update \
    && apk add openssl ca-certificates

# Create a non-privileged user
RUN addgroup -S docker && adduser -S docker -G docker

# Copy the build artifact from the build stage
COPY --from=builder /usr/src/app/target/release/marcus-radell-net /usr/local/bin/

# Set the user to run the application
USER docker

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/marcus-radell-net"]

# Expose port if your application listens on a port
EXPOSE 8080
