FROM scorpil/rust

ENV USER root

RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev

WORKDIR /weather-checker
RUN cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY . .
RUN cargo build --release
