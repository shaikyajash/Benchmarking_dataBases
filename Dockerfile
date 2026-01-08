# ============================================
# STAGE 1: Build Stage
# ============================================
FROM rust:bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    clang \
    llvm \
    lld \
    cmake \
    libclang-dev \
    libsnappy-dev \
    liblz4-dev \
    libzstd-dev \
    libbz2-dev \
    zlib1g-dev \
    librocksdb-dev \
    libleveldb-dev \
    && rm -rf /var/lib/apt/lists/*

# Set up environment
ENV CC=clang
ENV CXX=clang++

# Create a new empty project for dependency caching
WORKDIR /app

# Copy only dependency files first (Docker layer caching optimization)
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to compile dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build dependencies only (this layer gets cached)
RUN cargo build --release || true

# Remove the dummy build artifacts
RUN rm -rf src target/release/deps/data_bases*

# Now copy the actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release

# Strip debug symbols to reduce binary size
RUN strip target/release/data_bases 2>/dev/null || true

# ============================================
# STAGE 2: Runtime Stage
# ============================================
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libsnappy1v5 \
    liblz4-1 \
    libzstd1 \
    libbz2-1.0 \
    && rm -rf /var/lib/apt/lists/*

# Create app directory and data directories
WORKDIR /app
RUN mkdir -p /app/data/rocksdb /app/data/leveldb

# Copy the binary
COPY --from=builder /app/target/release/data_bases /app/data_bases

# Copy users.json for benchmarking
COPY users.json /app/users.json

# Expose the application port
EXPOSE 3000

# Run the binary
CMD ["/app/data_bases"]
