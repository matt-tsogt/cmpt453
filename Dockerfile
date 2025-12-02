# ---------- Builder stage ----------
FROM rust:1.91 AS builder

WORKDIR /app

# Install protoc for prost/tonic build.rs
RUN apt-get update \
    && apt-get install -y protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY main/Cargo.toml main/Cargo.toml
COPY grpc/Cargo.toml grpc/Cargo.toml
COPY rest/Cargo.toml rest/Cargo.toml
COPY rest_json/Cargo.toml rest_json/Cargo.toml
COPY loadgen/Cargo.toml loadgen/Cargo.toml

# Pre-build deps for caching
RUN mkdir -p main/src grpc/src rest/src rest_json/src loadgen/src \
    && echo "fn main() {}" > main/src/main.rs \
    && echo "fn main() {}" > grpc/src/main.rs \
    && echo "fn main() {}" > rest/src/main.rs \
    && echo "fn main() {}" > rest_json/src/main.rs \
    && echo "fn main() {}" > loadgen/src/main.rs \
    && cargo build --release -p grpc -p rest -p rest_json -p loadgçen

# Copy real sources
COPY main/ main/
COPY grpc/ grpc/
COPY rest/ rest/
COPY rest_json/ rest_json/
COPY loadgen/ loadgen/

# Build actual binaries
RUN cargo build --release -p grpc -p rest -p rest_json -p loadgen

# ---------- Runtime stage ----------
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/grpc /usr/local/bin/grpc
COPY --from=builder /app/target/release/rest /usr/local/bin/rest
COPY --from=builder /app/target/release/rest_json /usr/local/bin/rest_json
COPY --from=builder /app/target/release/loadgen /usr/local/bin/loadgen

CMD ["bash"]
