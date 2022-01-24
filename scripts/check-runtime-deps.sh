#!/bin/bash

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

set -e -u -o pipefail

main () {
    check_os_deps
    check_python_deps
}

check_os_deps () {
    /bin/gunzip --version > /dev/null
    /usr/bin/sha256sum --version > /dev/null
}

check_python_deps () {
    python3 -c ''
}

main "$@"
