#!/bin/bash

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
LICENSE_HEADER_PATH="resources/license-header.txt"
INSERT_LICENSE_HEADER_IF_MISSING=true

set -e -u -o pipefail

main () {
    local file="$1"
    local path=$(dirname "$SCRIPT_DIR")/${LICENSE_HEADER_PATH}

    is_license_header_present "$file" "$path" ||
        (echo "[!] missing license header for ${file}" &&
            if [ $INSERT_LICENSE_HEADER_IF_MISSING = true ]; then
                insert_license_header "$file" "$path"
            fi &&
                exit 1)
}

is_license_header_present () {
    local file="$1"
    local path="$2"
    diff <(head -n $(cat "$path" | wc -l) "$file") <(cat "$path") \
        >/dev/null
}

insert_license_header () {
    local file="$1"
    local path="$2"

    echo -e "$(cat ${path})\n\n$(cat ${file})" > "$file"
}

main "$@"
