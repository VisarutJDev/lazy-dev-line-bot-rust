# Builder stage
FROM rust:1.91-bullseye as builder

# Create a new empty shell project
WORKDIR /usr/src/app
RUN cargo new --bin lazy-dev-line-bot-rust
WORKDIR /usr/src/app/lazy-dev-line-bot-rust

# Copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# # Build dependencies - this is the caching Docker layer for dependencies
# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#     cargo build --release \
#     && rm src/*.rs

# Copy source tree
COPY ./src ./src

# Build for release
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

# Final stage
FROM rust:1.91-slim-bullseye

# Install OpenSSL - required for HTTPS requests
RUN apt-get update \
    && apt-get install -y ca-certificates libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/lazy-dev-line-bot-rust/target/release/lazy-dev-line-bot-rust .

# Create a non-root user
RUN useradd -m -u 1001 appuser
USER appuser

# Set environment variables
ENV RUST_LOG=info

# Expose the port
EXPOSE 3000

# Run the binary
CMD ["./lazy-dev-line-bot-rust"]