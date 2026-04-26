# client-engine: Rust/WASM development environment
# Axiom Protocol - client-engine aspect

FROM rust:1.95-slim

# System dependencies
RUN apt-get update && apt-get install -y \
    curl \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# Install toolchain components
RUN rustup component add clippy

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install cargo-watch for continuous monitoring
RUN cargo install cargo-watch

# Install cargo-cucumber support (via cucumber crate - resolved at build time)
# Testing dependencies are declared in Cargo.toml

# Set working directory to mounted source
WORKDIR /app

# Default: watch for changes, run clippy and tests on each change
# Override CMD via docker exec for specific operations
CMD ["cargo", "watch", \
     "--why", \
     "-s", "cargo clippy -- -D warnings && cargo test", \
     "-w", "src"]