FROM rust:1.39-slim-stretch
COPY . /app
WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3
COPY --from=0 /app/target/x86_64-unknown-linux-musl/release/hello-http /
CMD ["/hello-http"]
