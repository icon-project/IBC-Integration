FROM rust:1.69.0-alpine3.16

ENV PATH=/opt/binaryen/bin:$PATH


RUN set -eux; \
    apk update; \
    apk add --no-cache \
        bash \
        curl \
        musl-dev \
        ; \
    BINARYEN_VERS=110; \
    BINARYEN_URL="https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERS}/binaryen-version_${BINARYEN_VERS}-x86_64-linux.tar.gz"; \
    ARCH="$(apk --print-arch)"; \
    case "${ARCH}" in \
        aarch64|armv8) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='673e336c81c65e6b16dcdede33f4cc9ed0f08bde1dbe7a935f113605292dc800' ;; \
        x86_64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='0b2f6c8f85a3d02fde2efc0ced4657869d73fccfce59defb4e8d29233116e6db' ;; \
        *) echo >&2 "unsupported architecture: ARCH"; exit 1 ;; \
    esac; \
    curl -LfsSo /tmp/binaryen.tar.gz ${BINARYEN_URL}; \
    cd /tmp; \
    mkdir -p /opt/binaryen; \
    cd /opt/binaryen; \
    tar -xf /tmp/binaryen.tar.gz --strip-components=1; \
    rm -rf /tmp/binaryen.tar.gz;\
    rustup component add clippy rustfmt;\
    rustup target add wasm32-unknown-unknown;

RUN cargo install cosmwasm-check@1.4.1 --locked;
