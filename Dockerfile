FROM rust:1.63-slim-bullseye

ENV ROCKET_ENV=production

COPY ./ ./

RUN cargo build --release

CMD ["./target/debug/helltfbot-v2-backend"]

