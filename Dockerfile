# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:latest AS builder

# Copy local code to the container image.
WORKDIR /usr/src/web5-indexer
COPY . .
COPY config.toml /.cargo/

RUN cargo build --release

FROM rust:latest
WORKDIR /usr/src/web5-indexer
COPY --from=builder /usr/src/web5-indexer/target/release/web5-indexer web5-indexer
# Run the web service on container startup with the same environment variables
CMD ["sh", "-c", "./web5-indexer"]
