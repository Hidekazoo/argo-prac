FROM rust:1.86-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/pipeline-cli /usr/local/bin/pipeline-cli
ENTRYPOINT ["pipeline-cli"]
