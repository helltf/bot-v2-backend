FROM rust:latest

RUN rm -rf /usr/v2-backend
WORKDIR /usr/v2-backend

ENV ROCKET_ENV=production
ENV RUST_BACKTRACE=full

COPY ./ ./

RUN cargo build --release

CMD ["./target/debug/helltfbot-v2-backend"]
