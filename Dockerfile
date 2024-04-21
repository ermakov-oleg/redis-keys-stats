FROM rust:1.77-slim-buster as builder

RUN USER=root cargo new --bin redis-keys-stats
WORKDIR ./redis-keys-stats
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/redis_keys_stats*

RUN cargo build --release


FROM debian:buster-slim
COPY --from=builder /redis-keys-stats/target/release/redis-keys-stats /usr/local/bin/redis-keys-stats

ENV RUST_LOG=info


ENTRYPOINT ["redis-keys-stats"]