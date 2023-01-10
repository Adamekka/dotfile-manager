FROM rust:latest AS builder

WORKDIR /dman

COPY . .

RUN cargo install --path .

FROM debian:latest

WORKDIR /usr/bin

COPY --from=builder /dman/target/release/dman .
COPY --from=builder /dman/target/release/dman-gui .

WORKDIR /root

CMD ["bash"]
