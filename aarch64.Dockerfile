###############################
##### Base Image
###############################
FROM rust:1.84.0-slim AS base_image

WORKDIR /app

RUN apt-get update -yq && \
    apt-get install -y --no-install-recommends \
    clang \
    llvm-dev \
    libclang-dev \
    musl-tools \
    pkg-config \
    libssl-dev \
    build-essential \
    protobuf-compiler \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Install target platform (Cross-Compilation)
RUN rustup target add aarch64-unknown-linux-musl

###############################
##### Builder Image
###############################
FROM base_image AS builder

# copy folders
COPY ./src/ ./src

# copy cargo files
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo fetch

ENV CC_aarch64_unknown_linux_musl=clang
ENV AR_aarch64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=rust-lld
ENV RUST_BACKTRACE=full
ENV PROTOC_INCLUDE=/usr/include

# Build the application
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --target aarch64-unknown-linux-musl --release && \
    cp /app/target/aarch64-unknown-linux-musl/release/klickhouse_example /app/klickhouse_example

###############################
##### Runtime Image
###############################
FROM scratch

# copy compiled application
COPY --from=builder /app/klickhouse_example /klickhouse_example
# Copy application config
COPY ./confik.toml .

# specify that the application is started as PID 1
ENTRYPOINT ["/klickhouse_example"]
