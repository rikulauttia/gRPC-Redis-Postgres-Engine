# Use an official Rust image as the base
FROM rust:latest as builder

# Install protoc
RUN apt-get update && \
    apt-get install -y protobuf-compiler

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Build the application
RUN cargo install --path .

# Use Debian Bookworm (newer version)
FROM debian:bookworm-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/high_performance_backend /usr/local/bin/high_performance_backend

# Expose the application port
EXPOSE 50051

# Run the application
CMD ["high_performance_backend"]