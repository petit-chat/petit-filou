FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
WORKDIR /src
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN strip -s /src/target/x86_64-unknown-linux-musl/release/pf

FROM scratch
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/pf /
CMD ["/pf"]

LABEL org.opencontainers.image.source=https://github.com/petit-chat/petit-filou
LABEL org.opencontainers.image.description="petit-filou searches for mp4 videos for a given wordpress website."
LABEL org.opencontainers.image.licenses=GPL-3.0-or-later