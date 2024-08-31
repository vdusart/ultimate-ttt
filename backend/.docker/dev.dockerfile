FROM rust:1.78-buster

WORKDIR /app

COPY . .

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run"] 
