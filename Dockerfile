FROM  ekidd/rust-musl-builder as builder

WORKDIR /home/rust/ 

RUN rustup toolchain install nightly && rustup default nightly && rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml .
COPY  Cargo.lock .

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo +nightly build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/release/deps/rusti*

COPY src src
RUN RUSTFLAGS=-Clinker=musl-gcc cargo +nightly build --release --target=x86_64-unknown-linux-musl --features "cache" --verbose

FROM alpine:latest

COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/rusti /bin/rusti

CMD ["rusti"]
