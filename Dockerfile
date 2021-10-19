FROM ubuntu:20.04
ARG VERSION=0.2.5
ENV VERSION=${VERSION}
RUN apt-get update && apt-get install -y wget
RUN wget https://github.com/crate-ci/committed/releases/download/v${VERSION}/committed-v${VERSION}-x86_64-unknown-linux-musl.tar.gz && \
    tar -xzvf committed-v${VERSION}-x86_64-unknown-linux-musl.tar.gz && \
    mv committed /usr/local/bin
ENTRYPOINT ["/usr/local/bin/committed"]
