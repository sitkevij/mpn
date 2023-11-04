FROM ubuntu:latest
LABEL org.label-schema.name="mpn" \
    org.label-schema.description="MPEG-4 media file inspector." \
    org.label-schema.url="https://hub.docker.com/r/sitkevij/mpn" \
    org.label-schema.usage="https://github.com/sitkevij/mpn" \
    org.label-schema.vcs-url="https://github.com/sitkevij/mpn" \
    org.label-schema.vendor="sitkevij" \
    org.label-schema.version="ubuntu-latest" \
    maintainer="https://github.com/sitkevij"

RUN apt update && apt upgrade -y
RUN apt install curl rustc cargo libssl-dev openssl pkg-config vim -y
ENV source $HOME/.cargo/env && PATH=/root/.cargo/bin:$PATH
COPY . .
RUN ls -lt
# @see https://rust-lang.github.io/rfcs/2789-sparse-index.html
RUN cargo -Z sparse-registry install --path .
