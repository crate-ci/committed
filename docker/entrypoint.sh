#!/bin/bash

set -eu

log() {
    echo -e "$1" >&2
}

CMD_NAME="committed"

if [[ -z $(which ${CMD_NAME} 2>/dev/null) ]]; then
    log "ERROR: '${CMD_NAME}' not found"
    exit 1
fi

COMMAND="${CMD_NAME}"

echo "Linting commits:"
git -c safe.directory=. log --color=always --graph --oneline HEAD~..HEAD^2
echo ""
echo "Against 'committed.toml':"
${COMMAND} --dump-config -
echo ""
echo "If this fails, don't sweat it. We're trying to encourage clear communication and not hinder contributions."
echo "If it is a reasonable issue and you lack time or feel uncomfortable fixing it yourself,"
echo "let us know and we can mentor or fix it."
${COMMAND} --color=always -vv --no-merge-commit HEAD~..HEAD^2
