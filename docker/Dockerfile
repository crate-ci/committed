FROM ubuntu:20.04
ARG VERSION=1.0.3
ENV VERSION=${VERSION}
RUN apt-get update && apt-get install -y wget git
RUN wget https://github.com/crate-ci/committed/releases/download/v${VERSION}/committed-v${VERSION}-x86_64-unknown-linux-musl.tar.gz && \
    tar -xzvf committed-v${VERSION}-x86_64-unknown-linux-musl.tar.gz && \
    mv committed /usr/local/bin
COPY entrypoint.sh /entrypoint.sh
WORKDIR /github/workspace
ENTRYPOINT ["/bin/bash", "/entrypoint.sh"]
