FROM rustlang/rust:nightly as builder

WORKDIR /zaryn_p2p

COPY .env .env

COPY diesel.toml diesel.toml

COPY . .

RUN cargo build --release

# RUN cargo install --path .

FROM debian:buster-slim

RUN mkdir zaryn_p2p

WORKDIR /zaryn_p2p

# install libpq
RUN apt-get update; \
    apt-get install -y --no-install-recommends libpq-dev; \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /zaryn_p2p/target/release/zaryn_p2p ./

COPY --from=builder /zaryn_p2p/diesel.toml .
COPY --from=builder /zaryn_p2p/.env .


ENTRYPOINT [ "/zaryn_p2p/zaryn_p2p" ]