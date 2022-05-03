FROM rust:1.60

WORKDIR /hello
COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo build --release

FROM debian:bullseye

COPY --from=0 /hello/target/release/hello_world .
EXPOSE 8080

CMD ["./hello_world"]
