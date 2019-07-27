FROM rust:latest as builder

WORKDIR /src/rusti

ENV USER rust

RUN cargo init

COPY Cargo.toml .
COPY  Cargo.lock .

RUN cargo build --release
RUN rm -f target/release/deps/rusti*

COPY src src
RUN cargo build --release --features "cache"

FROM alpine:latest

COPY --from=builder /src/rusti/target/release/rusti /usr/local/bin/rusti

CMD ["rusti"]
