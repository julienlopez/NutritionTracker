FROM rust:latest

WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch --locked

EXPOSE 8080

COPY . .

CMD [ "cargo", "watch", "--why", "-x", "run", "--ignore", "database_data/", "--poll" ]
