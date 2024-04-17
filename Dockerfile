FROM rust:1.77-slim

COPY . /app
WORKDIR /app

RUN apt update && apt install -y pkg-config libssl-dev openssl
RUN cargo build -r

ENTRYPOINT [ "cargo" ]
CMD ["run", "--release", "--quiet"]
