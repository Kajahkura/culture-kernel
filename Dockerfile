# Stage 1: Build the Kernel
FROM rust:1.84 as builder
WORKDIR /app
COPY . .
# We build in release mode for maximum speed
RUN cargo build --release

# Stage 2: Run the Kernel (Tiny Image)
FROM debian:bookworm-slim
WORKDIR /app
# Install OpenSSL (Required for API)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary and seed file
COPY --from=builder /app/target/release/culture-kernel /app/culture-kernel
COPY --from=builder /app/rituals.json /app/rituals.json

# Run the 'serve' command by default
CMD ["./culture-kernel", "serve", "--port", "8080"]
