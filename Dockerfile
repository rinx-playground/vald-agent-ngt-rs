FROM ekidd/rust-musl-builder AS builder

USER root

RUN apt update \
    && apt install -y \
    build-essential \
    cmake \
    clang \
    gcc \
    g++ \
    libclang-dev \
    llvm-dev \
    && apt autoclean -y \
    && apt autoremove -y \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt

USER rust

WORKDIR /home/rust/vald-agent-ngt-rs

COPY Makefile .
COPY build.rs .
COPY Cargo.toml .
COPY Cargo.lock .

WORKDIR /home/rust/vald-agent-ngt-rs/src

COPY src .

WORKDIR /home/rust/vald-agent-ngt-rs

ENV RUSTFLAGS='-L/usr/local/musl/lib -L/usr/lib/x86_64-linux-musl -L/lib/x86_64-linux-musl -C linker=musl-gcc -Clink-arg=-fuse-ld=gold'
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_ALL_STATIC=true
ENV OPENSSL_STATIC=true
ENV LIBZ_SYS_STATIC=1

RUN make RUSTFLAGS="${RUSTFLAGS}" build/release

# FROM gcr.io/distroless/static AS base
FROM alpine:edge AS base

COPY --from=builder /home/rust/vald-agent-ngt-rs/target/x86_64-unknown-linux-musl/release/vald-agent-ngt-rs /vald-agent-ngt-rs

ENTRYPOINT [ "/vald-agent-ngt-rs" ]
