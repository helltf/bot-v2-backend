FROM rust:latest

WORKDIR /usr/v2-backend

ENV ROCKET_ENV=production
ENV RUST_BACKTRACE=full

COPY . .

RUN cargo clean
RUN cargo build --release

CMD ["./target/release/helltfbot-v2-backend"]
