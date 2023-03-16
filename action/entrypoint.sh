#!/bin/bash

set -eu

log() {
    echo -e "$1" >&2
}

CMD_NAME="committed"

if [[ -z $(which ${CMD_NAME} 2>/dev/null) ]]; then
    VERSION=1.0.14
    log "Downloading 'committed' v${VERSION}"
    wget https://github.com/crate-ci/committed/releases/download/v${VERSION}/committed-v${VERSION}-x86_64-unknown-linux-musl.tar.gz
    sudo tar -xzvf committed-v${VERSION}-x86_64-unknown-linux-musl.tar.gz -C /usr/local/bin ./committed
    rm committed-v${VERSION}-x86_64-unknown-linux-musl.tar.gz
fi

COMMAND="${CMD_NAME}"

echo "Linting commits:"
git config --global --add safe.directory "$PWD"
git log --color=always --graph --oneline "${INPUT_COMMITS}" || true
echo ""
echo "Against 'committed.toml':"
${COMMAND} --dump-config - || true
echo ""
echo "If this fails, don't sweat it. We're trying to encourage clear communication and not hinder contributions."
echo "If it is a reasonable issue and you lack time or feel uncomfortable fixing it yourself,"
echo "let us know and we can mentor or fix it."
${COMMAND} --color=always $INPUT_ARGS "${INPUT_COMMITS}"
