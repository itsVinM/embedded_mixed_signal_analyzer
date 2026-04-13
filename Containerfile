FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
      curl ca-certificates git \
      pkg-config libusb-1.0-0-dev libudev-dev \
      build-essential \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --no-modify-path --default-toolchain nightly

ENV PATH="/root/.cargo/bin:${PATH}"
ENV CARGO_HOME=/usr/local/cargo

RUN rustup target add thumbv7em-none-eabihf \
    && rustup component add rust-src rustfmt clippy llvm-tools \
    && cargo install flip-link

WORKDIR /workspace