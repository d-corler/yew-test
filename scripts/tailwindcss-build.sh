#!/usr/bin/env bash

DIR=$(dirname "$0")
ARCH=$(uname -m)

TMP_DIR=$DIR/../tmp
TOOLS_DIR=$TMP_DIR/tools
NODE_DIR=$TMP_DIR/node

mkdir -p $TOOLS_DIR
mkdir -p $NODE_DIR

if ! [ -f $TOOLS_DIR/tailwind ]; then
    echo "missing tailwindcss binary, downloading..."

    if [ "$(uname)" == "Darwin" ]; then
        curl https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-$ARCH -sLo $TOOLS_DIR/tailwind
    else
        printf '%s\n' "OS not yet supported" >&2
    fi

    chmod +x $TOOLS_DIR/tailwind
fi

$TOOLS_DIR/tailwind -i $DIR/../src/styles/index.scss -o $TMP_DIR/styles/index.css --postcss
