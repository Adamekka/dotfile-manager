FROM rust:latest AS builder

WORKDIR /dman

COPY . .

RUN cargo install --path . --bin dman
RUN cargo install --path . --bin dman-gui


FROM debian

WORKDIR /usr/bin

COPY --from=builder /dman/target/release/dman .
COPY --from=builder /dman/target/release/dman-gui .

WORKDIR /root

CMD ["bash"]
