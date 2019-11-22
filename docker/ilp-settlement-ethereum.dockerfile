# Build Settlement Engines into standalone binary
FROM clux/muslrust:stable as rust
ARG CARGO_BUILD_OPTION=""
ARG RUST_BIN_DIR_NAME="debug"

RUN echo "Building profile: ${CARGO_BUILD_OPTION}, output dir: ${RUST_BIN_DIR_NAME}"

WORKDIR /usr/src
COPY ./Cargo.toml /usr/src/Cargo.toml
COPY ./Cargo.lock /usr/src/Cargo.lock
COPY ./crates /usr/src/crates

RUN cargo build ${CARGO_BUILD_OPTION} --package ilp-settlement-ethereum --bin ilp-settlement-ethereum

# Deploy compiled binary to another container
FROM alpine
ARG CARGO_BUILD_OPTION=""
ARG RUST_BIN_DIR_NAME="debug"

# Expose ports for HTTP
# - 3000: HTTP - settlement
EXPOSE 3000

# Install SSL certs
RUN apk --no-cache add ca-certificates

# Copy binary
COPY --from=rust \
    /usr/src/target/x86_64-unknown-linux-musl/${RUST_BIN_DIR_NAME}/ilp-settlement-ethereum \
    /usr/local/bin/ilp-settlement-ethereum

WORKDIR /opt/app

# ENV RUST_BACKTRACE=1
ENV RUST_LOG=interledger=debug

ENTRYPOINT [ "/usr/local/bin/ilp-settlement-ethereum" ]
CMD [ ]
