FROM rust:1.69-buster as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./

RUN cargo build --release
RUN rm src/*.rs

COPY . .
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/local/bin

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-blog-backend .

EXPOSE 8080

CMD ["./rust-blog-backend"]
