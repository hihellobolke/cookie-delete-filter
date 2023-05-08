# ------------------------------------------------------------------------------
# Build Stage
# ------------------------------------------------------------------------------

FROM alpine:3.16 as wasm-builder

ARG RUSTC_VERSION=1.58.1
RUN apk update \
    && apk upgrade \
    && apk add build-base binutils-gold openssl3-dev protoc curl \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- --no-modify-path --profile minimal --default-toolchain ${RUSTC_VERSION} \
      -c rustfmt -t wasm32-unknown-unknown -y

WORKDIR /usr/src/wasm-builder

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY src src

RUN source $HOME/.cargo/env \
    && cargo build --target=wasm32-unknown-unknown --release

# ------------------------------------------------------------------------------
# Run Stage
# ------------------------------------------------------------------------------

FROM scratch

COPY --from=wasm-shim-build /usr/src/wasm-shim/target/wasm32-unknown-unknown/release/cookie_delete_filter.wasm /plugin.wasm
