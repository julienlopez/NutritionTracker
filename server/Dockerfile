FROM rust:latest

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch --locked

WORKDIR /app/server

EXPOSE 8080

COPY . .

RUN ["cat", "Cargo.toml"]
CMD [ "cargo", "watch", "--why", "-x", "run", "--ignore", "migrations/", "--poll" ]
