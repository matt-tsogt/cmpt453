# BUILD STAGE
FROM rust:1.91 AS builder

WORKDIR /app

# install protoc (for tonic/prost)
RUN apt-get update \
    && apt-get install -y protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# --- copy only what this experiment needs ---

# workspace manifest + lockfile
COPY Cargo.toml Cargo.lock ./

# crate manifests for dependency caching
COPY main/Cargo.toml main/Cargo.toml
COPY experiment_one/Cargo.toml experiment_one/Cargo.toml
COPY experiment_one/grpc_simple/Cargo.toml experiment_one/grpc_simple/Cargo.toml
COPY experiment_one/rest_simple/Cargo.toml experiment_one/rest_simple/Cargo.toml
COPY experiment_one/loadgen_simple/Cargo.toml experiment_one/loadgen_simple/Cargo.toml

# prebuild deps with dummy main.rs to maximize caching
RUN mkdir -p \
      main/src \
      experiment_one/src \
      experiment_one/grpc_simple/src \
      experiment_one/rest_simple/src \
      experiment_one/loadgen_simple/src \
  && echo 'fn main() {}' > main/src/main.rs \
  && echo 'fn main() {}' > experiment_one/grpc_simple/src/main.rs \
  && echo 'fn main() {}' > experiment_one/rest_simple/src/main.rs \
  && echo 'fn main() {}' > experiment_one/loadgen_simple/src/main.rs \
  && cargo build --release -p grpc_simple -p rest_simple -p loadgen_simple || true

# now copy real sources
COPY main/ main/
COPY experiment_one/ experiment_one/

# build actual binaries
RUN cargo build --release -p grpc_simple -p rest_simple -p loadgen_simple


# RUNTIME STAGE
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/grpc_simple   /usr/local/bin/grpc_simple
COPY --from=builder /app/target/release/rest_simple   /usr/local/bin/rest_simple
COPY --from=builder /app/target/release/loadgen_simple /usr/local/bin/loadgen_simple

CMD ["bash"]
