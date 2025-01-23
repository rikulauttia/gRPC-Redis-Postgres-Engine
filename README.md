# High Performance Backend

A high-performance backend service built with Rust, gRPC, Redis, and PostgreSQL.

## Features

- Fast message processing with gRPC
- Redis caching for improved performance
- PostgreSQL persistence for data storage
- Docker support for easy deployment
- Async Rust for high concurrency

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Redis
- Docker and Docker Compose (for containerized deployment)
- Protocol Buffers compiler (protoc)

## Local Development Setup

1. Install dependencies:

```bash
# For macOS
brew install postgresql redis protobuf
```

2. Clone the repository:

```bash
git clone https://github.com/rikulauttia/gRPC-Redis-Postgres-Engine.git
cd high_performance_backend
```

3. Set up environment variables:

```bash
# Create .env file
cp .env.example .env
# Edit .env with your configuration
```

4. Run the database migrations:

```bash
cargo install sqlx-cli
sqlx migrate run
```

5. Build and run the service:

```bash
cargo build
cargo run
```

## Docker Deployment

1. Build and start the containers:

```bash
docker compose up --build
```

2. Stop the containers:

```bash
docker compose down
```

## API Documentation

### MessagingService

#### SendMessage

Sends a message to a specific user.
Request (MessageRequest):

-user_id: string - Unique identifier of the user

-message: string - Content of the message

Response (MessageResponse):

-status: string - Status of the operation

Example using grpcurl:

```bash
grpcurl -d '{"user_id": "123", "message": "Hello"}' \
  -plaintext localhost:50051 \
  messaging.MessagingService/SendMessage
```

### Performance Testing

Run performance tests using ghz:

```bash
ghz --insecure \
  --proto ./proto/service.proto \
  --call messaging.MessagingService.SendMessage \
  -d '{"user_id": "123", "message": "test"}' \
  -n 1000 \
  -c 50 \
  '[::1]:50051'
```
