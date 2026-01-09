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
    VERSION=1.1.10
    if [[ "$(uname -m)" == "arm64" || "$(uname -m)" == "aarch64" ]]; then
        ARCH="aarch64"
    else
        ARCH="x86_64"
    fi
    UNAME=$(uname -s)
    if [[ "$UNAME" == "Darwin" ]]; then
        TARGET_FILE="${ARCH}-apple-darwin"
        FILE_EXT="tar.gz"
    elif [[ "$UNAME" == CYGWIN* || "$UNAME" == MINGW* || "$UNAME" == MSYS* ]] ; then
        TARGET_FILE="${ARCH}-pc-windows-msvc"
        FILE_EXT="zip"
    else
        TARGET_FILE="${ARCH}-unknown-linux-musl"
        FILE_EXT="tar.gz"
    fi
    FILE_NAME="${CMD_NAME}-v${VERSION}-${TARGET_FILE}.${FILE_EXT}"
    log "Downloading '${CMD_NAME}' v${VERSION}"
    wget --progress=dot:mega "https://github.com/crate-ci/committed/releases/download/v${VERSION}/${FILE_NAME}"
    mkdir -p ${_INSTALL_DIR}
    if [[ "$FILE_EXT" == "zip" ]]; then
        unzip -o "${FILE_NAME}" -d ${_INSTALL_DIR} ${CMD_NAME}.exe
    else
        tar -xzvf "${FILE_NAME}" -C ${_INSTALL_DIR} ./${CMD_NAME}
    fi
    rm "${FILE_NAME}"
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
