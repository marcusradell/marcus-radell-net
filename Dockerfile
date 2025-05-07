# Build stage
FROM rust:1.86.0-alpine3.21 AS builder

WORKDIR /usr/src/app

RUN apk add --no-cache pkgconfig openssl-dev libc-dev

# Copy the entire project
COPY . .

RUN cargo build --release

# Production stage - using Alpine for minimal image size
FROM alpine:3.21

RUN apk update \
    && apk add --no-cache openssl ca-certificates

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
