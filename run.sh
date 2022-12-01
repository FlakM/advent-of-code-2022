#!/usr/bin/env bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

if [[ $# -eq 0 ]] ; then
    echo 'Pass a day number ie day1a'
    exit 1
fi

DIR="$SCRIPT_DIR/$1"

if [ -d "$DIR" ]; then
    cd "$DIR"
    cargo clippy -q
    cargo build -q --release
    time cargo -q run --release
    cd -
else
    echo "Solution for $1 does not exist!"
fi

