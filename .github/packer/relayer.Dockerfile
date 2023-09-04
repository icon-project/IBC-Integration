FROM alpine:latest AS build-env

RUN apk add --no-cache tar

ARG VERSION=0.1.0-alpha.8
ARG BASE_URL=https://github.com/icon-project/ibc-relay/releases/download
ARG PLATFORM=linux
ARG ARCH=amd64

ADD $BASE_URL/v${VERSION}/ibc-relay_${VERSION}_${PLATFORM}_${ARCH}.tar.gz .

RUN tar -xvf ibc-relay_${VERSION}_linux_amd64.tar.gz && \
  mv ibc-relay_${VERSION}_linux_amd64/relayer . && \
  rm -rf ibc-relay_${VERSION}_linux_amd64*

FROM scratch

COPY --from=build-env /relayer /usr/local/bin/relayer

ENTRYPOINT ["/usr/local/bin/relayer"]