FROM alpine:latest AS build-env

RUN apk add --no-cache tar

ARG VERSION=0.9.3
ARG BASE_URL=https://github.com/icon-project/ibc-relay/releases/download

ADD $BASE_URL/untagged-550ac853d4c7acdc8701/ibc-relay_${VERSION}_linux_amd64.tar.gz .

RUN tar -xvf ibc-relay_${VERSION}_linux_amd64.tar.gz && \
  mv ibc-relay_${VERSION}_linux_amd64/relayer . && \
  rm -rf ibc-relay_${VERSION}_linux_amd64*

FROM scratch

COPY --from=build-env /relayer /usr/local/bin/relayer

ENTRYPOINT ["/usr/local/bin/relayer"]