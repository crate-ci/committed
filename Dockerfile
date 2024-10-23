ARG DEBIAN_DIST=bullseye

FROM rust:${DEBIAN_DIST} as builder
WORKDIR /usr/src/committed
COPY . .
RUN cargo install --path ./crates/committed

FROM debian:${DEBIAN_DIST}-slim
COPY --from=builder /usr/local/cargo/bin/committed /usr/local/bin/committed
ENTRYPOINT ["committed"]
CMD ["--help"]
