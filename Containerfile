FROM ubuntu:24.04

RUN apt-get update && apt-get install -y --no-install-recommends \
      curl ca-certificates git \
      pkg-config libusb-1.0-0-dev libudev-dev \
      build-essential \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --default-toolchain nightly

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add thumbv7em-none-eabihf \
    && rustup component add rust-src llvm-tools \
    && cargo install flip-link cargo-flash cargo-binutils