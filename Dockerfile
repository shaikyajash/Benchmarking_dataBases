
# STAGE 1: Build Stage
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

ENV CC=clang
ENV CXX=clang++

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src && echo "fn main() {}" > src/main.rs

RUN cargo build --release || true

RUN rm -rf src target/release/deps/data_bases*

COPY src ./src

RUN cargo build --release

RUN strip target/release/data_bases 2>/dev/null || true

# STAGE 2: Runtime Stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libsnappy1v5 \
    liblz4-1 \
    libzstd1 \
    libbz2-1.0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN mkdir -p /app/data/rocksdb /app/data/leveldb

# Copy the binary
COPY --from=builder /app/target/release/data_bases /app/data_bases

COPY users.json /app/users.json

EXPOSE 3000
ENTRYPOINT ["/app/data_bases"]