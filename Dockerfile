# Dockerfile for Hermetic, Reproducible Static Compilation of SSS_CHAIN
FROM rust:1.78.0-slim AS builder

WORKDIR /usr/src/sss-chain
COPY . .

RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && apt-get install -y musl-tools && \
    cargo build --release --bin sss_chain --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /usr/src/sss-chain/target/x86_64-unknown-linux-musl/release/sss_chain /sss_chain
ENTRYPOINT ["/sss_chain"]
