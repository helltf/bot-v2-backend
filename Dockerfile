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

ENV DATABASE_URL=postgresql://postgres:1234@helltfbot-v2_db_1:5432/twitch

CMD ["./target/release/helltfbot-v2-backend"]
