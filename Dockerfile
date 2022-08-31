FROM rust:1-buster

WORKDIR /usr/v2-backend

ENV ROCKET_ENV=production
ENV RUST_BACKTRACE=1
COPY ./src ./src
COPY Cargo.lock . 
COPY Cargo.toml .
COPY Rocket.toml .
COPY diesel.toml .

RUN cargo build --release
EXPOSE 8080

CMD ["./target/release/helltfbot-v2-backend"]
