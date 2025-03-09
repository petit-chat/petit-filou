FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --locked -r
COPY . .
RUN cargo build --locked -r

FROM scratch
COPY --from=builder /app/target/release/pf /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/pf"]

LABEL org.opencontainers.image.source=https://github.com/petit-chat/petit-filou
LABEL org.opencontainers.image.description="petit-filou scans wordpress websites to find videos"
LABEL org.opencontainers.image.licenses=GPL-3.0-or-later
