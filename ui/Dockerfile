FROM rust:latest

WORKDIR /app

RUN cargo install dioxus-cli

EXPOSE 8080

COPY . .

CMD [ "dx", "serve", "--platform", "fullstack", "--hot-reload" ]
