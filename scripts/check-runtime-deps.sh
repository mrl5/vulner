#!/bin/bash

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

set -e -u -o pipefail

main () {
    check_python_deps
}

check_python_deps () {
    python3 -c 'import jsonschema'
}

main "$@"
