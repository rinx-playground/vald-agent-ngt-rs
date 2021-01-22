FROM rust:latest AS builder

RUN apt update \
    && apt install -y \
    build-essential \
    cmake \
    clang \
    g++ \
    gcc \
    git \
    libclang-dev \
    llvm-dev \
    && apt autoclean -y \
    && apt autoremove -y \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt

WORKDIR /app

COPY Makefile .
COPY build.rs .
COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN make build/release

FROM gcr.io/distroless/cc AS base

COPY --from=builder /app/target/release/vald-agent-ngt-rs /vald-agent-ngt-rs
COPY --from=builder /app/target/release/build/ngt-sys-*/out/lib/* /usr/local/lib/

ENV LD_LIBRARY_PATH=/usr/local/lib

ENTRYPOINT [ "/vald-agent-ngt-rs" ]
