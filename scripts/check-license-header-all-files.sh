#!/bin/bash

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

set -e -u -o pipefail

main () {
    local e=0
    local main_dir=$(dirname "$SCRIPT_DIR")
    local covered_files=$(ls "$main_dir"/crates/**/src/*.rs && ls "$main_dir"/crates/**/src/**/*.rs)

    for file in $covered_files; do \
        ./scripts/check-license-header.sh "$file" || ((e+=1)); \
    done
    if [ $e -gt 0 ]; then exit 1; fi
}

main "$@"
