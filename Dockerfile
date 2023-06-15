FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]
