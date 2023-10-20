# Use the official Rust image as a parent image
FROM rust:1.59 AS builder

# Create a new directory in the container
WORKDIR /usr/src/rust_rocket_web_api

# Copy the local package to the container's directory
COPY . .

# Build the application in release mode
RUN cargo build --release

# Start a new build stage
FROM debian:buster-slim

# Copy the binary from the builder stage
COPY --from=builder /usr/src/rust_rocket_web_api/target/release/rust_rocket_web_api /usr/local/bin/

# Set the command to run the binary
CMD ["rust_rocket_web_api"]