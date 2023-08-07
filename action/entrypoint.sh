#!/bin/bash

set -eu

log() {
    echo -e "$1" >&2
}

_DEFAULT_INSTALL_DIR=${HOME}/bin
_INSTALL_DIR=${INSTALL_DIR:-${_DEFAULT_INSTALL_DIR}}
CMD_NAME="committed"
COMMAND="${_INSTALL_DIR}/${CMD_NAME}"

if [[ -z ${INPUT_COMMITS:-} ]]; then
    log "'INPUT_COMMITS' is required"
    exit 1
fi

if [[ ! -x ${COMMAND} ]]; then
    VERSION=1.0.20
    log "Downloading 'committed' v${VERSION}"
    wget --progress=dot:mega https://github.com/crate-ci/committed/releases/download/v${VERSION}/committed-v${VERSION}-x86_64-unknown-linux-musl.tar.gz
    mkdir -p ${_INSTALL_DIR}
    tar -xzvf committed-v${VERSION}-x86_64-unknown-linux-musl.tar.gz -C ${_INSTALL_DIR} ./${CMD_NAME}
    rm committed-v${VERSION}-x86_64-unknown-linux-musl.tar.gz
fi

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
${COMMAND} --color=always ${INPUT_ARGS:-} "${INPUT_COMMITS}"
