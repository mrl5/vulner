#!/bin/bash

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

set -e -u -o pipefail

main () {
    check_os_deps
    check_python_deps
}

check_os_deps () {
    command -v /bin/gunzip > /dev/null
}

check_python_deps () {
    # https://unix.stackexchange.com/a/340695
    command -v python3-config > /dev/null
}

main "$@"
